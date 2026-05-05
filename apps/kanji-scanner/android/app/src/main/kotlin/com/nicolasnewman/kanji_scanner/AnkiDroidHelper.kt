package com.nicolasnewman.kanji_scanner

import CardInfo
import ModelInfo
import NoteWithFields
import android.content.ContentResolver
import android.content.Context
import android.content.pm.PackageManager
import android.net.Uri
import androidx.core.content.ContextCompat
import com.ichi2.anki.FlashCardsContract.Card
import com.ichi2.anki.FlashCardsContract.Model
import com.ichi2.anki.FlashCardsContract.Note
import com.ichi2.anki.api.AddContentApi
import java.util.Locale

data class BaseCardInfo(
    val noteId: Long,
    val cardOrd: Int,
    val deckId: Long,
    val question: String,
    val answer: String,
    val reps: Int,
    val lapses: Int,
    val type: Int
)

class AnkiDroidHelper(val context: Context) {
    companion object {
        // Field separator used in Anki notes
        private const val FIELD_SEPARATOR = "\u001f"
    }

    private val mContext: Context = context.applicationContext
    private val mResolver: ContentResolver = mContext.contentResolver
    private val mApi = AddContentApi(mContext)

    private val noteUri: Uri =
        if (mApi.apiHostSpecVersion == 2) Note.CONTENT_URI_V2 else Note.CONTENT_URI

    fun hasPermission(): Boolean {
        return ContextCompat.checkSelfPermission(
            mContext,
            AddContentApi.READ_WRITE_PERMISSION
        ) == PackageManager.PERMISSION_GRANTED
    }

    fun getDecks(): Map<Long, String>? {
        if (!hasPermission()) {
            return mapOf()
        }
        return mApi.deckList
    }

    fun getModels(): Map<Long, String>? {
        if (!hasPermission()) {
            return mapOf()
        }
        return mApi.modelList
    }

    /**
     * Get detailed information about all models
     * @return List of ModelInfo containing name, ID, and note count
     */
    fun getModelsWithInfo(): List<ModelInfo> {
        if (!hasPermission()) {
            return emptyList()
        }

        val cursor = mResolver.query(
            Model.CONTENT_URI,
            arrayOf(Model._ID, Model.NAME, Model.NOTE_COUNT, Model.FIELD_NAMES),
            null,
            null,
            null
        ) ?: return emptyList()

        val models = mutableListOf<ModelInfo>()
        cursor.use { cursor ->
            val idIdx = cursor.getColumnIndex(Model._ID)
            val nameIdx = cursor.getColumnIndex(Model.NAME)
            val noteCountIdx = cursor.getColumnIndex(Model.NOTE_COUNT)
            val fieldNamesIndex = cursor.getColumnIndex(Model.FIELD_NAMES)

            if (idIdx < 0 || nameIdx < 0 || noteCountIdx < 0 || fieldNamesIndex < 0) {
                return emptyList()
            }

            while (cursor.moveToNext()) {
                val modelId = cursor.getLong(idIdx)
                val modelName = cursor.getString(nameIdx) ?: "Unknown"
                val noteCount = cursor.getLong(noteCountIdx)
                val fields = cursor.getString(fieldNamesIndex).split(FIELD_SEPARATOR)

                models.add(ModelInfo(modelId, modelName, noteCount, fields))
            }
        }

        return models
    }

    /**
     * Get field names for a specific model
     */
    fun getModelFieldNames(modelId: Long): List<String> {
        if (!hasPermission()) {
            return emptyList()
        }
        println(modelId)
        val modelUri = Uri.withAppendedPath(Model.CONTENT_URI, modelId.toString())
        println(modelUri)
        val cursor = mResolver.query(modelUri, Model.DEFAULT_PROJECTION, null, null, null)
            ?: return emptyList()

        return cursor.use { cursor ->
            if (cursor.moveToFirst()) {
                val fieldNamesIndex = cursor.getColumnIndex(Model.FIELD_NAMES)
                println(fieldNamesIndex)
                if (fieldNamesIndex >= 0) {
                    val fieldNamesString = cursor.getString(fieldNamesIndex)
                    // Field names are separated by \u001f
                    fieldNamesString.split(FIELD_SEPARATOR)
                } else {
                    emptyList()
                }
            } else {
                emptyList()
            }
        }
    }

    /**
     * Get all notes for a specific model with their fields mapped by field name
     */
    fun getNotesWithFieldsForModel(modelId: Long, fieldName: String): List<NoteWithFields> {
        if (!hasPermission()) {
            return emptyList()
        }

        val fieldNames = mApi.getFieldList(modelId)

        if (fieldNames.isNullOrEmpty()) {
            return emptyList()
        }

        val selection = String.format(Locale.US, "%s=%d", Note.MID, modelId)
        val cursor = mResolver.query(
            noteUri,
            arrayOf(Note._ID, Note.MID, Note.FLDS, Note.TAGS),
            selection,
            null,
            null
        ) ?: return emptyList()

        val notes = mutableListOf<NoteWithFields>()
        cursor.use { cursor ->
            val noteIdIdx = cursor.getColumnIndex(Note._ID)
            val midIdx = cursor.getColumnIndex(Note.MID)
            val fldsIdx = cursor.getColumnIndex(Note.FLDS)
            val tagsIdx = cursor.getColumnIndex(Note.TAGS)

            if (noteIdIdx < 0 || midIdx < 0 || fldsIdx < 0 || tagsIdx < 0) {
                return emptyList()
            }

            while (cursor.moveToNext()) {
                val noteId = cursor.getLong(noteIdIdx)
                val mid = cursor.getLong(midIdx)
                val fldsString = cursor.getString(fldsIdx)
                val tagsString = cursor.getString(tagsIdx) ?: ""

                val fieldValues = fldsString.split(FIELD_SEPARATOR)

                var kanji: String? = null;
                for (i in fieldNames.indices) {
                    if (i < fieldValues.size && fieldNames[i].equals(fieldName)) {
                        kanji = fieldValues[i]
                    }
                }

                // Parse tags (space-separated, trimmed)
                val tags = tagsString.trim().split("\\s+".toRegex()).filter { it.isNotEmpty() }

                if (!kanji.isNullOrEmpty()) {
                    notes.add(NoteWithFields(noteId, mid, kanji, tags))
                }
            }
        }

        return notes
    }

    /**
     * Get all cards for a specific note
     */
    fun getCardsForNote(noteId: Long): List<BaseCardInfo> {
        if (!hasPermission()) {
            return emptyList()
        }

        val noteUriWithId = Uri.withAppendedPath(Note.CONTENT_URI, noteId.toString())
        val cardsUri = Uri.withAppendedPath(noteUriWithId, "cards")
        val cursor = mResolver.query(
            cardsUri,
            arrayOf(
                Card.NOTE_ID,
                Card.CARD_ORD,
                Card.CARD_NAME,
                Card.DECK_ID,
                Card.QUESTION,
                Card.ANSWER,
                Card.REPS,
                Card.LAPSES,
                Card.TYPE),
            null,
            null,
            null
        ) ?: return emptyList()

        val cards = mutableListOf<BaseCardInfo>()
        cursor.use { cursor ->
            val noteIdIdx = cursor.getColumnIndex(Card.NOTE_ID)
            val cardOrdIdx = cursor.getColumnIndex(Card.CARD_ORD)
            val deckIdIdx = cursor.getColumnIndex(Card.DECK_ID)
            val questionIdx = cursor.getColumnIndex(Card.QUESTION)
            val answerIdx = cursor.getColumnIndex(Card.ANSWER)
            val repsIdx = cursor.getColumnIndex(Card.REPS)
            val lapsesIdx = cursor.getColumnIndex(Card.LAPSES)
            val typeIdx = cursor.getColumnIndex(Card.TYPE)

            if (noteIdIdx < 0 || cardOrdIdx < 0 || deckIdIdx < 0 || questionIdx < 0 || answerIdx < 0) {
                return emptyList()
            }

            while (cursor.moveToNext()) {
                val cardNoteId = cursor.getLong(noteIdIdx)
                val cardOrd = cursor.getInt(cardOrdIdx)
                val deckId = cursor.getLong(deckIdIdx)
                val question = cursor.getString(questionIdx) ?: ""
                val answer = cursor.getString(answerIdx) ?: ""
                val reps = cursor.getInt(repsIdx)
                val lapses = cursor.getInt(lapsesIdx)
                val type = cursor.getInt(typeIdx)

                cards.add(BaseCardInfo(cardNoteId, cardOrd, deckId, question, answer, reps, lapses, type))
            }
        }

        return cards
    }

    /**
     * Get all cards for notes of a specific model
     */
    fun getCardsForModel(modelId: Long, fieldName: String, offset: Long, limit: Long): List<CardInfo> {
        if (!hasPermission()) {
            return emptyList()
        }

        val result = arrayListOf<CardInfo>()
        val fieldNames = mApi.getFieldList(modelId)

        if (fieldNames.isNullOrEmpty()) {
            return emptyList()
        }

        val selection = String.format(Locale.US, "%s=%d", Note.MID, modelId)
        val cursor = mResolver.query(
            noteUri,
            arrayOf(Note._ID, Note.MID, Note.FLDS, Note.TAGS),
            selection,
            null,
            "${Note._ID} ASC LIMIT $limit OFFSET $offset"
        ) ?: return emptyList()

        cursor.use { cursor ->
            val noteIdIdx = cursor.getColumnIndex(Note._ID)
            val midIdx = cursor.getColumnIndex(Note.MID)
            val fldsIdx = cursor.getColumnIndex(Note.FLDS)
            val tagsIdx = cursor.getColumnIndex(Note.TAGS)

            if (noteIdIdx < 0 || midIdx < 0 || fldsIdx < 0 || tagsIdx < 0) {
                return emptyList()
            }

            while (cursor.moveToNext()) {
                val noteId = cursor.getLong(noteIdIdx)
                val mid = cursor.getLong(midIdx)
                val fldsString = cursor.getString(fldsIdx)
                val tagsString = cursor.getString(tagsIdx) ?: ""

                val fieldValues = fldsString.split(FIELD_SEPARATOR)
                var kanji: String? = null
                for (i in fieldNames.indices) {
                    if (i < fieldValues.size && fieldNames[i].equals(fieldName)) {
                        kanji = fieldValues[i]
                        break
                    }
                }

                if (kanji.isNullOrEmpty()) continue

                val tags = tagsString.trim().split("\\s+".toRegex()).filter { it.isNotEmpty() }
                val cards = getCardsForNote(noteId)

                for (card in cards) {
                    if (card.reps > 0) {
                        result.add(
                            CardInfo(
                                noteId,
                                card.cardOrd.toLong(),
                                card.deckId,
                                card.question,
                                card.answer,
                                mid,
                                kanji,
                                tags,
                                card.reps.toLong(),
                                card.lapses.toLong(),
                                card.type.toLong()
                            )
                        )
                    }
                }
            }
        }

        return result
    }
}