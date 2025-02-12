

def camel_to_snake(string):
    return ''.join([(f"_{c.lower()}" if c.isupper() else c) for c in string])

def snake_to_camel(string):
    return ''.join([(word.capitalize() if i != 0 else word) for i, word in enumerate(string.lower().split('_'))])

def size_to_type(size):
    if size == 1:
        return 'bool'
    if size <= 8:
        return 'u8'
    if size <= 16:
        return 'u16'
    else:
        return 'u32'

def sanitize_name(name):
    if name in RUST_KEYWORDS:
        return f"r#{name}"
    return name

RUST_KEYWORDS = set([
    "as", "break", "const", "continue", "crate", "else", "enum", 
    "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait",
    "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "abstract", "become", "box",
    "do", "final", "macro", "override", "priv", "typeof",
    "unsized", "virtual", "yield", "try",
    "macro_rules", "union", "'static",
])