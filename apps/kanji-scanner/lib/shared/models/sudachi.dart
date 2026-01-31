// TODO: Remove this file once Sudachi API is fully integrated.
// class SudachiResponse {
//   final List<String>? response;

//   const SudachiResponse({required this.response});

//   factory SudachiResponse.fromJson(Map<String, dynamic>? json) {
//     if (json == null) {
//       return SudachiResponse(response: null);
//     }

//     final response = (json['response'] as List)
//         .map((e) => List<String>.from(e as List))
//         .toList();

//     return SudachiResponse(response: response[0]);
//   }
// }
