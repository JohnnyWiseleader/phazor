#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 27
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 18
#define ALIAS_COUNT 0
#define TOKEN_COUNT 12
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 10
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_ATroute = 1,
  anon_sym_ATprops = 2,
  anon_sym_def = 3,
  anon_sym_LPAREN = 4,
  anon_sym_RPAREN = 5,
  anon_sym_COLON = 6,
  aux_sym_route_string_token1 = 7,
  anon_sym_COMMA = 8,
  sym_identifier = 9,
  sym_html_block = 10,
  sym_comment = 11,
  sym_source_file = 12,
  sym_statement = 13,
  sym_route_string = 14,
  sym_prop_list = 15,
  aux_sym_source_file_repeat1 = 16,
  aux_sym_prop_list_repeat1 = 17,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_ATroute] = "@route",
  [anon_sym_ATprops] = "@props",
  [anon_sym_def] = "def",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_COLON] = ":",
  [aux_sym_route_string_token1] = "route_string_token1",
  [anon_sym_COMMA] = ",",
  [sym_identifier] = "identifier",
  [sym_html_block] = "html_block",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym_statement] = "statement",
  [sym_route_string] = "route_string",
  [sym_prop_list] = "prop_list",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_prop_list_repeat1] = "prop_list_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_ATroute] = anon_sym_ATroute,
  [anon_sym_ATprops] = anon_sym_ATprops,
  [anon_sym_def] = anon_sym_def,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_COLON] = anon_sym_COLON,
  [aux_sym_route_string_token1] = aux_sym_route_string_token1,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [sym_identifier] = sym_identifier,
  [sym_html_block] = sym_html_block,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym_statement] = sym_statement,
  [sym_route_string] = sym_route_string,
  [sym_prop_list] = sym_prop_list,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_prop_list_repeat1] = aux_sym_prop_list_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_ATroute] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATprops] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_def] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_route_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_html_block] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_route_string] = {
    .visible = true,
    .named = true,
  },
  [sym_prop_list] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_prop_list_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(20);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '#') ADVANCE(31);
      if (lookahead == '(') ADVANCE(24);
      if (lookahead == ')') ADVANCE(25);
      if (lookahead == ',') ADVANCE(28);
      if (lookahead == ':') ADVANCE(26);
      if (lookahead == '<') ADVANCE(18);
      if (lookahead == '@') ADVANCE(12);
      if (lookahead == 'd') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(27);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '#') ADVANCE(31);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 3:
      if (lookahead == '/') ADVANCE(19);
      END_STATE();
    case 4:
      if (lookahead == '<') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '>') ADVANCE(4);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '>') ADVANCE(30);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 9:
      if (lookahead == 'f') ADVANCE(23);
      END_STATE();
    case 10:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 11:
      if (lookahead == 'o') ADVANCE(13);
      END_STATE();
    case 12:
      if (lookahead == 'p') ADVANCE(14);
      if (lookahead == 'r') ADVANCE(10);
      END_STATE();
    case 13:
      if (lookahead == 'p') ADVANCE(15);
      END_STATE();
    case 14:
      if (lookahead == 'r') ADVANCE(11);
      END_STATE();
    case 15:
      if (lookahead == 's') ADVANCE(22);
      END_STATE();
    case 16:
      if (lookahead == 't') ADVANCE(8);
      END_STATE();
    case 17:
      if (lookahead == 'u') ADVANCE(16);
      END_STATE();
    case 18:
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(5);
      END_STATE();
    case 19:
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(6);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_ATroute);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_ATprops);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_def);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(aux_sym_route_string_token1);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_html_block);
      if (lookahead == '<') ADVANCE(18);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(30);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(31);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_ATroute] = ACTIONS(1),
    [anon_sym_ATprops] = ACTIONS(1),
    [anon_sym_def] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [aux_sym_route_string_token1] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [sym_html_block] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(20),
    [sym_statement] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_ATroute] = ACTIONS(7),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
    ACTIONS(11), 1,
      anon_sym_ATroute,
    STATE(2), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
  [14] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_ATroute,
    ACTIONS(14), 1,
      ts_builtin_sym_end,
    STATE(2), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
  [28] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(16), 1,
      anon_sym_def,
    ACTIONS(18), 1,
      anon_sym_COMMA,
    STATE(4), 1,
      aux_sym_prop_list_repeat1,
  [41] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_def,
    ACTIONS(23), 1,
      anon_sym_COMMA,
    STATE(4), 1,
      aux_sym_prop_list_repeat1,
  [54] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(23), 1,
      anon_sym_COMMA,
    ACTIONS(25), 1,
      anon_sym_def,
    STATE(5), 1,
      aux_sym_prop_list_repeat1,
  [67] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(27), 1,
      anon_sym_ATprops,
    ACTIONS(29), 1,
      anon_sym_def,
  [77] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(31), 1,
      sym_identifier,
    STATE(15), 1,
      sym_prop_list,
  [87] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      aux_sym_route_string_token1,
    STATE(7), 1,
      sym_route_string,
  [97] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(35), 2,
      anon_sym_ATprops,
      anon_sym_def,
  [105] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(16), 2,
      anon_sym_def,
      anon_sym_COMMA,
  [113] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 2,
      ts_builtin_sym_end,
      anon_sym_ATroute,
  [121] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 2,
      ts_builtin_sym_end,
      anon_sym_ATroute,
  [129] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 1,
      sym_identifier,
  [136] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(43), 1,
      anon_sym_def,
  [143] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 1,
      anon_sym_LPAREN,
  [150] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(47), 1,
      sym_identifier,
  [157] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 1,
      sym_identifier,
  [164] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(51), 1,
      anon_sym_RPAREN,
  [171] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(53), 1,
      ts_builtin_sym_end,
  [178] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(55), 1,
      anon_sym_LPAREN,
  [185] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(57), 1,
      anon_sym_COLON,
  [192] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(59), 1,
      anon_sym_RPAREN,
  [199] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(61), 1,
      sym_html_block,
  [206] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(63), 1,
      anon_sym_COLON,
  [213] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(65), 1,
      sym_html_block,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 14,
  [SMALL_STATE(4)] = 28,
  [SMALL_STATE(5)] = 41,
  [SMALL_STATE(6)] = 54,
  [SMALL_STATE(7)] = 67,
  [SMALL_STATE(8)] = 77,
  [SMALL_STATE(9)] = 87,
  [SMALL_STATE(10)] = 97,
  [SMALL_STATE(11)] = 105,
  [SMALL_STATE(12)] = 113,
  [SMALL_STATE(13)] = 121,
  [SMALL_STATE(14)] = 129,
  [SMALL_STATE(15)] = 136,
  [SMALL_STATE(16)] = 143,
  [SMALL_STATE(17)] = 150,
  [SMALL_STATE(18)] = 157,
  [SMALL_STATE(19)] = 164,
  [SMALL_STATE(20)] = 171,
  [SMALL_STATE(21)] = 178,
  [SMALL_STATE(22)] = 185,
  [SMALL_STATE(23)] = 192,
  [SMALL_STATE(24)] = 199,
  [SMALL_STATE(25)] = 206,
  [SMALL_STATE(26)] = 213,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [11] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(9),
  [14] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [16] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_prop_list_repeat1, 2),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_prop_list_repeat1, 2), SHIFT_REPEAT(17),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_prop_list, 2),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_prop_list, 1),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_route_string, 1),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 8),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 10),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [53] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_phz(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
