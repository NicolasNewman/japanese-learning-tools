use anyhow::Error as E;
use clap::{Parser};

use candle_core::{DType, Tensor};
use candle_examples::token_output_stream::TokenOutputStream;
use candle_nn::VarBuilder;
use candle_transformers::models::marian;

use tokenizers::Tokenizer;

#[derive(Parser)]
pub struct Args {
    /// Run on CPU rather than on GPU.
    #[arg(long)]
    pub cpu: bool,

    /// Text to be translated
    #[arg(long)]
    pub text: String,
}

const MODEL: &'static [u8] = include_bytes!("../../../../packages/opus-mt-ja-en/dist/model.safetensors");
const TOKENIZER: &'static [u8] = include_bytes!("../../../../packages/opus-mt-ja-en/dist/tokenizer-marian-base-ja.json");
const TOKENIZER_DEC: &'static [u8] = include_bytes!("../../../../packages/opus-mt-ja-en/dist/tokenizer-marian-base-en.json");

pub fn run_model(args: Args) -> anyhow::Result<String> {
    let config = marian::Config {
        activation_function: candle_nn::Activation::Swish,
        d_model: 512,
        decoder_attention_heads: 8,
        decoder_ffn_dim: 2048,
        decoder_layers: 6,
        decoder_start_token_id: 60715,
        decoder_vocab_size: Some(60716),
        encoder_attention_heads: 8,
        encoder_ffn_dim: 2048,
        encoder_layers: 6,
        eos_token_id: 0,
        forced_eos_token_id: 0,
        is_encoder_decoder: true,
        max_position_embeddings: 512,
        pad_token_id: 60715,
        scale_embedding: true,
        share_encoder_decoder_embeddings: true,
        use_cache: true,
        vocab_size: 60716,
    };
    
    let tokenizer = Tokenizer::from_bytes(TOKENIZER).map_err(E::msg)?;

    let tokenizer_dec = Tokenizer::from_bytes(TOKENIZER_DEC).map_err(E::msg)?;
    let mut tokenizer_dec = TokenOutputStream::new(tokenizer_dec);

    let device = candle_examples::device(args.cpu)?;

    let vb = VarBuilder::from_slice_safetensors(MODEL, DType::F32, &device)?;
    let mut model = marian::MTModel::new(&config, vb)?;

    let mut logits_processor =
        candle_transformers::generation::LogitsProcessor::new(1337, None, None);

    let encoder_xs = {
        let mut tokens = tokenizer
            .encode(args.text, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        tokens.push(config.eos_token_id);
        let tokens = Tensor::new(tokens.as_slice(), &device)?.unsqueeze(0)?;
        model.encoder().forward(&tokens, 0)?
    };

    let mut token_ids = vec![config.decoder_start_token_id];
    let mut translation = String::new();
    for index in 0..1000 {
        let context_size = if index >= 1 { 1 } else { token_ids.len() };
        let start_pos = token_ids.len().saturating_sub(context_size);
        let input_ids = Tensor::new(&token_ids[start_pos..], &device)?.unsqueeze(0)?;
        let logits = model.decode(&input_ids, &encoder_xs, start_pos)?;
        let logits = logits.squeeze(0)?;
        let logits = logits.get(logits.dim(0)? - 1)?;
        let token = logits_processor.sample(&logits)?;
        token_ids.push(token);
        if let Some(t) = tokenizer_dec.next_token(token)? {
            translation.push_str(&t);
        }
        if token == config.eos_token_id || token == config.forced_eos_token_id {
            break;
        }
    }

    Ok(translation)
}