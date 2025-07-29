#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 19
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 17
#define ALIAS_COUNT 0
#define TOKEN_COUNT 11
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_ATroute = 1,
  anon_sym_def = 2,
  anon_sym_LPAREN = 3,
  anon_sym_RPAREN = 4,
  anon_sym_COLON = 5,
  sym_string = 6,
  sym_identifier = 7,
  sym_open_tag = 8,
  sym_close_tag = 9,
  sym_inner_html = 10,
  sym_source_file = 11,
  sym_statement = 12,
  sym_html_block = 13,
  sym_html_element = 14,
  aux_sym_source_file_repeat1 = 15,
  aux_sym_html_block_repeat1 = 16,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_ATroute] = "@route",
  [anon_sym_def] = "def",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_COLON] = ":",
  [sym_string] = "string",
  [sym_identifier] = "identifier",
  [sym_open_tag] = "open_tag",
  [sym_close_tag] = "close_tag",
  [sym_inner_html] = "inner_html",
  [sym_source_file] = "source_file",
  [sym_statement] = "statement",
  [sym_html_block] = "html_block",
  [sym_html_element] = "html_element",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_html_block_repeat1] = "html_block_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_ATroute] = anon_sym_ATroute,
  [anon_sym_def] = anon_sym_def,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_string] = sym_string,
  [sym_identifier] = sym_identifier,
  [sym_open_tag] = sym_open_tag,
  [sym_close_tag] = sym_close_tag,
  [sym_inner_html] = sym_inner_html,
  [sym_source_file] = sym_source_file,
  [sym_statement] = sym_statement,
  [sym_html_block] = sym_html_block,
  [sym_html_element] = sym_html_element,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_html_block_repeat1] = aux_sym_html_block_repeat1,
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
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_open_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_close_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_inner_html] = {
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
  [sym_html_block] = {
    .visible = true,
    .named = true,
  },
  [sym_html_element] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_html_block_repeat1] = {
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(16);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '(') ADVANCE(19);
      if (lookahead == ')') ADVANCE(20);
      if (lookahead == ':') ADVANCE(21);
      if (lookahead == '<') ADVANCE(3);
      if (lookahead == '@') ADVANCE(11);
      if (lookahead == 'd') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(22);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '/') ADVANCE(15);
      END_STATE();
    case 3:
      if (lookahead == '/') ADVANCE(15);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(5);
      END_STATE();
    case 4:
      if (lookahead == '<') ADVANCE(2);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(26);
      if (lookahead != 0) ADVANCE(27);
      END_STATE();
    case 5:
      if (lookahead == '>') ADVANCE(24);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '>') ADVANCE(25);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(17);
      END_STATE();
    case 9:
      if (lookahead == 'f') ADVANCE(18);
      END_STATE();
    case 10:
      if (lookahead == 'o') ADVANCE(13);
      END_STATE();
    case 11:
      if (lookahead == 'r') ADVANCE(10);
      END_STATE();
    case 12:
      if (lookahead == 't') ADVANCE(8);
      END_STATE();
    case 13:
      if (lookahead == 'u') ADVANCE(12);
      END_STATE();
    case 14:
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(14)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(23);
      END_STATE();
    case 15:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(6);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_ATroute);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_def);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_open_tag);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_close_tag);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_inner_html);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(26);
      if (lookahead != 0 &&
          lookahead != '<') ADVANCE(27);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_inner_html);
      if (lookahead != 0 &&
          lookahead != '<') ADVANCE(27);
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
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 4},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 14},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_ATroute] = ACTIONS(1),
    [anon_sym_def] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_open_tag] = ACTIONS(1),
    [sym_close_tag] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(12),
    [sym_statement] = STATE(4),
    [aux_sym_source_file_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_ATroute] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(9), 1,
      sym_open_tag,
    ACTIONS(7), 2,
      ts_builtin_sym_end,
      anon_sym_ATroute,
    STATE(3), 2,
      sym_html_element,
      aux_sym_html_block_repeat1,
  [12] = 3,
    ACTIONS(13), 1,
      sym_open_tag,
    ACTIONS(11), 2,
      ts_builtin_sym_end,
      anon_sym_ATroute,
    STATE(3), 2,
      sym_html_element,
      aux_sym_html_block_repeat1,
  [24] = 3,
    ACTIONS(5), 1,
      anon_sym_ATroute,
    ACTIONS(16), 1,
      ts_builtin_sym_end,
    STATE(5), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
  [35] = 3,
    ACTIONS(18), 1,
      ts_builtin_sym_end,
    ACTIONS(20), 1,
      anon_sym_ATroute,
    STATE(5), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
  [46] = 3,
    ACTIONS(9), 1,
      sym_open_tag,
    STATE(10), 1,
      sym_html_block,
    STATE(2), 2,
      sym_html_element,
      aux_sym_html_block_repeat1,
  [57] = 1,
    ACTIONS(23), 3,
      ts_builtin_sym_end,
      anon_sym_ATroute,
      sym_open_tag,
  [63] = 1,
    ACTIONS(25), 3,
      ts_builtin_sym_end,
      anon_sym_ATroute,
      sym_open_tag,
  [69] = 2,
    ACTIONS(27), 1,
      sym_close_tag,
    ACTIONS(29), 1,
      sym_inner_html,
  [76] = 1,
    ACTIONS(31), 2,
      ts_builtin_sym_end,
      anon_sym_ATroute,
  [81] = 1,
    ACTIONS(33), 1,
      sym_string,
  [85] = 1,
    ACTIONS(35), 1,
      ts_builtin_sym_end,
  [89] = 1,
    ACTIONS(37), 1,
      anon_sym_def,
  [93] = 1,
    ACTIONS(39), 1,
      sym_identifier,
  [97] = 1,
    ACTIONS(41), 1,
      anon_sym_LPAREN,
  [101] = 1,
    ACTIONS(43), 1,
      anon_sym_RPAREN,
  [105] = 1,
    ACTIONS(45), 1,
      anon_sym_COLON,
  [109] = 1,
    ACTIONS(47), 1,
      sym_close_tag,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 12,
  [SMALL_STATE(4)] = 24,
  [SMALL_STATE(5)] = 35,
  [SMALL_STATE(6)] = 46,
  [SMALL_STATE(7)] = 57,
  [SMALL_STATE(8)] = 63,
  [SMALL_STATE(9)] = 69,
  [SMALL_STATE(10)] = 76,
  [SMALL_STATE(11)] = 81,
  [SMALL_STATE(12)] = 85,
  [SMALL_STATE(13)] = 89,
  [SMALL_STATE(14)] = 93,
  [SMALL_STATE(15)] = 97,
  [SMALL_STATE(16)] = 101,
  [SMALL_STATE(17)] = 105,
  [SMALL_STATE(18)] = 109,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_html_block, 1),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_html_block_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_html_block_repeat1, 2), SHIFT_REPEAT(9),
  [16] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [18] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [20] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(11),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_html_element, 2),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_html_element, 3),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 8),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [35] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
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
