module.exports = grammar({
  name: 'phz',

  extras: $ => [$.comment, /\s/],

  rules: {
    source_file: $ => repeat($.statement),

    statement: $ => seq(
      '@route',
      $.route_string,
      optional(seq(
        '@props',
        $.prop_list
      )),
      'def',
      $.identifier,
      '(', ')',
      ':',
      $.html_block
    ),

    // Allow route string to contain named parameters like ":id"
    route_string: $ => /"[^"]*"/,

    prop_list: $ => sep1($.identifier, ','),

    string: $ => /"[^"]*"/,
    identifier: $ => /[a-zA-Z_]\w*/,
    html_block: $ => /(<[^>]+>[^<]*<\/[^>]+>\s*)+/,

    comment: _ => /#[^\n]*/,
  }
});

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}
