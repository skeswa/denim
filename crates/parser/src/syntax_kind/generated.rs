//! Generated by `cargo xtask gen grammar`, do not edit by hand.

#![allow(bad_style, dead_code, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    AMP,
    AMP2,
    AMPEQ,
    AT,
    BANG,
    CARET,
    CARETEQ,
    COLON,
    COLON2,
    COMMA,
    DOLLAR,
    DOT,
    DOT2,
    DOT2EQ,
    DOT3,
    EQ,
    EQ2,
    FAT_ARROW,
    GTEQ,
    LTEQ,
    L_ANGLE,
    L_BRACK,
    L_CURLY,
    L_PAREN,
    MINUS,
    MINUSEQ,
    NEQ,
    PERCENT,
    PERCENTEQ,
    PIPE,
    PIPE2,
    PIPEEQ,
    PLUS,
    PLUSEQ,
    POUND,
    QUESTION,
    R_ANGLE,
    R_BRACK,
    R_CURLY,
    R_PAREN,
    SEMICOLON,
    SHL,
    SHLEQ,
    SHR,
    SHREQ,
    SLASH,
    SLASHEQ,
    STAR,
    STAREQ,
    THIN_ARROW,
    TILDE,
    UNDERSCORE,
    SELF_TYPE_KW,
    ABSTRACT_KW,
    AS_KW,
    ASYNC_KW,
    AWAIT_KW,
    BECOME_KW,
    BOX_KW,
    BREAK_KW,
    CONST_KW,
    CONTINUE_KW,
    CRATE_KW,
    DO_KW,
    ELSE_KW,
    ENUM_KW,
    EXTERN_KW,
    FALSE_KW,
    FINAL_KW,
    FN_KW,
    FOR_KW,
    GEN_KW,
    IF_KW,
    IMPL_KW,
    IN_KW,
    LET_KW,
    LOOP_KW,
    MACRO_KW,
    MATCH_KW,
    MOD_KW,
    MOVE_KW,
    MUT_KW,
    OVERRIDE_KW,
    PRIV_KW,
    PUB_KW,
    REF_KW,
    RETURN_KW,
    SELF_KW,
    STATIC_KW,
    STRUCT_KW,
    SUPER_KW,
    TRAIT_KW,
    TRUE_KW,
    TRY_KW,
    TYPE_KW,
    TYPEOF_KW,
    UNSAFE_KW,
    UNSIZED_KW,
    USE_KW,
    VIRTUAL_KW,
    WHERE_KW,
    WHILE_KW,
    YIELD_KW,
    BUILTIN_KW,
    OFFSET_OF_KW,
    FORMAT_ARGS_KW,
    ASM_KW,
    MACRO_RULES_KW,
    UNION_KW,
    DEFAULT_KW,
    RAW_KW,
    DYN_KW,
    AUTO_KW,
    YEET_KW,
    BYTE,
    BYTE_STRING,
    CHAR,
    C_STRING,
    FLOAT_NUMBER,
    INT_NUMBER,
    RAW_BYTE_STRING,
    RAW_C_STRING,
    RAW_STRING,
    STRING,
    COMMENT,
    ERROR,
    IDENT,
    LIFETIME_IDENT,
    NEWLINE,
    SHEBANG,
    WHITESPACE,
    ABI,
    ADT,
    ARG_LIST,
    ARRAY_EXPR,
    ARRAY_TYPE,
    ASM_EXPR,
    ASSOC_ITEM,
    ASSOC_ITEM_LIST,
    ASSOC_TYPE_ARG,
    ATTR,
    AWAIT_EXPR,
    BECOME_EXPR,
    BIN_EXPR,
    BLOCK_EXPR,
    BOX_PAT,
    BREAK_EXPR,
    CALL_EXPR,
    CAST_EXPR,
    CLOSURE_BINDER,
    CLOSURE_EXPR,
    CONST,
    CONST_ARG,
    CONST_BLOCK_PAT,
    CONST_PARAM,
    CONTINUE_EXPR,
    DYN_TRAIT_TYPE,
    ENUM,
    EXPR,
    EXPR_STMT,
    EXTERN_BLOCK,
    EXTERN_CRATE,
    EXTERN_ITEM,
    EXTERN_ITEM_LIST,
    FIELD_EXPR,
    FIELD_LIST,
    FN,
    FN_PTR_TYPE,
    FORMAT_ARGS_ARG,
    FORMAT_ARGS_EXPR,
    FOR_EXPR,
    FOR_TYPE,
    GENERIC_ARG,
    GENERIC_ARG_LIST,
    GENERIC_PARAM,
    GENERIC_PARAM_LIST,
    IDENT_PAT,
    IF_EXPR,
    IMPL,
    IMPL_TRAIT_TYPE,
    INDEX_EXPR,
    INFER_TYPE,
    ITEM,
    ITEM_LIST,
    LABEL,
    LET_ELSE,
    LET_EXPR,
    LET_STMT,
    LIFETIME,
    LIFETIME_ARG,
    LIFETIME_PARAM,
    LITERAL,
    LITERAL_PAT,
    LOOP_EXPR,
    MACRO_CALL,
    MACRO_DEF,
    MACRO_EXPR,
    MACRO_ITEMS,
    MACRO_PAT,
    MACRO_RULES,
    MACRO_STMTS,
    MACRO_TYPE,
    MATCH_ARM,
    MATCH_ARM_LIST,
    MATCH_EXPR,
    MATCH_GUARD,
    META,
    METHOD_CALL_EXPR,
    MODULE,
    NAME,
    NAME_REF,
    NEVER_TYPE,
    OFFSET_OF_EXPR,
    OR_PAT,
    PARAM,
    PARAM_LIST,
    PAREN_EXPR,
    PAREN_PAT,
    PAREN_TYPE,
    PAT,
    PATH,
    PATH_EXPR,
    PATH_PAT,
    PATH_SEGMENT,
    PATH_TYPE,
    PREFIX_EXPR,
    PTR_TYPE,
    RANGE_EXPR,
    RANGE_PAT,
    RECORD_EXPR,
    RECORD_EXPR_FIELD,
    RECORD_EXPR_FIELD_LIST,
    RECORD_FIELD,
    RECORD_FIELD_LIST,
    RECORD_PAT,
    RECORD_PAT_FIELD,
    RECORD_PAT_FIELD_LIST,
    REF_EXPR,
    REF_PAT,
    REF_TYPE,
    RENAME,
    REST_PAT,
    RETURN_EXPR,
    RET_TYPE,
    SELF_PARAM,
    SLICE_PAT,
    SLICE_TYPE,
    SOURCE_FILE,
    STATIC,
    STMT,
    STMT_LIST,
    STRUCT,
    TOKEN_TREE,
    TRAIT,
    TRAIT_ALIAS,
    TRY_EXPR,
    TUPLE_EXPR,
    TUPLE_FIELD,
    TUPLE_FIELD_LIST,
    TUPLE_PAT,
    TUPLE_STRUCT_PAT,
    TUPLE_TYPE,
    TYPE,
    TYPE_ALIAS,
    TYPE_ARG,
    TYPE_BOUND,
    TYPE_BOUND_LIST,
    TYPE_PARAM,
    UNDERSCORE_EXPR,
    UNION,
    USE,
    USE_TREE,
    USE_TREE_LIST,
    VARIANT,
    VARIANT_LIST,
    VISIBILITY,
    WHERE_CLAUSE,
    WHERE_PRED,
    WHILE_EXPR,
    WILDCARD_PAT,
    YEET_EXPR,
    YIELD_EXPR,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    #[doc = r" Checks whether this syntax kind is a strict keyword for the given edition."]
    #[doc = r" Strict keywords are identifiers that are always considered keywords."]
    pub fn is_strict_keyword(self) -> bool {
        matches!(
            self,
            SELF_TYPE_KW
                | ABSTRACT_KW
                | AS_KW
                | ASYNC_KW
                | AWAIT_KW
                | BECOME_KW
                | BOX_KW
                | BREAK_KW
                | CONST_KW
                | CONTINUE_KW
                | CRATE_KW
                | DO_KW
                | ELSE_KW
                | ENUM_KW
                | EXTERN_KW
                | FALSE_KW
                | FINAL_KW
                | FN_KW
                | FOR_KW
                | GEN_KW
                | IF_KW
                | IMPL_KW
                | IN_KW
                | LET_KW
                | LOOP_KW
                | MACRO_KW
                | MATCH_KW
                | MOD_KW
                | MOVE_KW
                | MUT_KW
                | OVERRIDE_KW
                | PRIV_KW
                | PUB_KW
                | REF_KW
                | RETURN_KW
                | SELF_KW
                | STATIC_KW
                | STRUCT_KW
                | SUPER_KW
                | TRAIT_KW
                | TRUE_KW
                | TRY_KW
                | TYPE_KW
                | TYPEOF_KW
                | UNSAFE_KW
                | UNSIZED_KW
                | USE_KW
                | VIRTUAL_KW
                | WHERE_KW
                | WHILE_KW
                | YIELD_KW
        )
    }
    #[doc = r" Checks whether this syntax kind is a strict or weak keyword for the given edition."]
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            SELF_TYPE_KW
                | ABSTRACT_KW
                | AS_KW
                | ASYNC_KW
                | AWAIT_KW
                | BECOME_KW
                | BOX_KW
                | BREAK_KW
                | CONST_KW
                | CONTINUE_KW
                | CRATE_KW
                | DO_KW
                | ELSE_KW
                | ENUM_KW
                | EXTERN_KW
                | FALSE_KW
                | FINAL_KW
                | FN_KW
                | FOR_KW
                | GEN_KW
                | IF_KW
                | IMPL_KW
                | IN_KW
                | LET_KW
                | LOOP_KW
                | MACRO_KW
                | MATCH_KW
                | MOD_KW
                | MOVE_KW
                | MUT_KW
                | OVERRIDE_KW
                | PRIV_KW
                | PUB_KW
                | REF_KW
                | RETURN_KW
                | SELF_KW
                | STATIC_KW
                | STRUCT_KW
                | SUPER_KW
                | TRAIT_KW
                | TRUE_KW
                | TRY_KW
                | TYPE_KW
                | TYPEOF_KW
                | UNSAFE_KW
                | UNSIZED_KW
                | USE_KW
                | VIRTUAL_KW
                | WHERE_KW
                | WHILE_KW
                | YIELD_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self,
            AMP | AMP2
                | AMPEQ
                | AT
                | BANG
                | CARET
                | CARETEQ
                | COLON
                | COLON2
                | COMMA
                | DOLLAR
                | DOT
                | DOT2
                | DOT2EQ
                | DOT3
                | EQ
                | EQ2
                | FAT_ARROW
                | GTEQ
                | LTEQ
                | L_ANGLE
                | L_BRACK
                | L_CURLY
                | L_PAREN
                | MINUS
                | MINUSEQ
                | NEQ
                | PERCENT
                | PERCENTEQ
                | PIPE
                | PIPE2
                | PIPEEQ
                | PLUS
                | PLUSEQ
                | POUND
                | QUESTION
                | R_ANGLE
                | R_BRACK
                | R_CURLY
                | R_PAREN
                | SEMICOLON
                | SHL
                | SHLEQ
                | SHR
                | SHREQ
                | SLASH
                | SLASHEQ
                | STAR
                | STAREQ
                | THIN_ARROW
                | TILDE
                | UNDERSCORE
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            BYTE | BYTE_STRING
                | CHAR
                | C_STRING
                | FLOAT_NUMBER
                | INT_NUMBER
                | RAW_BYTE_STRING
                | RAW_C_STRING
                | RAW_STRING
                | STRING
        )
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "Self" => SELF_TYPE_KW,
            "abstract" => ABSTRACT_KW,
            "as" => AS_KW,
            "async" => ASYNC_KW,
            "await" => AWAIT_KW,
            "become" => BECOME_KW,
            "box" => BOX_KW,
            "break" => BREAK_KW,
            "const" => CONST_KW,
            "continue" => CONTINUE_KW,
            "crate" => CRATE_KW,
            "do" => DO_KW,
            "else" => ELSE_KW,
            "enum" => ENUM_KW,
            "extern" => EXTERN_KW,
            "false" => FALSE_KW,
            "final" => FINAL_KW,
            "fn" => FN_KW,
            "for" => FOR_KW,
            "gen" => GEN_KW,
            "if" => IF_KW,
            "impl" => IMPL_KW,
            "in" => IN_KW,
            "let" => LET_KW,
            "loop" => LOOP_KW,
            "macro" => MACRO_KW,
            "match" => MATCH_KW,
            "mod" => MOD_KW,
            "move" => MOVE_KW,
            "mut" => MUT_KW,
            "override" => OVERRIDE_KW,
            "priv" => PRIV_KW,
            "pub" => PUB_KW,
            "ref" => REF_KW,
            "return" => RETURN_KW,
            "self" => SELF_KW,
            "static" => STATIC_KW,
            "struct" => STRUCT_KW,
            "super" => SUPER_KW,
            "trait" => TRAIT_KW,
            "true" => TRUE_KW,
            "try" => TRY_KW,
            "type" => TYPE_KW,
            "typeof" => TYPEOF_KW,
            "unsafe" => UNSAFE_KW,
            "unsized" => UNSIZED_KW,
            "use" => USE_KW,
            "virtual" => VIRTUAL_KW,
            "where" => WHERE_KW,
            "while" => WHILE_KW,
            "yield" => YIELD_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '&' => AMP,
            '@' => AT,
            '!' => BANG,
            '^' => CARET,
            ':' => COLON,
            ',' => COMMA,
            '$' => DOLLAR,
            '.' => DOT,
            '=' => EQ,
            '<' => L_ANGLE,
            '[' => L_BRACK,
            '{' => L_CURLY,
            '(' => L_PAREN,
            '-' => MINUS,
            '%' => PERCENT,
            '|' => PIPE,
            '+' => PLUS,
            '#' => POUND,
            '?' => QUESTION,
            '>' => R_ANGLE,
            ']' => R_BRACK,
            '}' => R_CURLY,
            ')' => R_PAREN,
            ';' => SEMICOLON,
            '/' => SLASH,
            '*' => STAR,
            '~' => TILDE,
            '_' => UNDERSCORE,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [&] => { $ crate :: SyntaxKind :: AMP } ; [&&] => { $ crate :: SyntaxKind :: AMP2 } ; [&=] => { $ crate :: SyntaxKind :: AMPEQ } ; [@] => { $ crate :: SyntaxKind :: AT } ; [!] => { $ crate :: SyntaxKind :: BANG } ; [^] => { $ crate :: SyntaxKind :: CARET } ; [^=] => { $ crate :: SyntaxKind :: CARETEQ } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [::] => { $ crate :: SyntaxKind :: COLON2 } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; [$] => { $ crate :: SyntaxKind :: DOLLAR } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [..] => { $ crate :: SyntaxKind :: DOT2 } ; [..=] => { $ crate :: SyntaxKind :: DOT2EQ } ; [...] => { $ crate :: SyntaxKind :: DOT3 } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [==] => { $ crate :: SyntaxKind :: EQ2 } ; [=>] => { $ crate :: SyntaxKind :: FAT_ARROW } ; [>=] => { $ crate :: SyntaxKind :: GTEQ } ; [<=] => { $ crate :: SyntaxKind :: LTEQ } ; [<] => { $ crate :: SyntaxKind :: L_ANGLE } ; ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; ['{'] => { $ crate :: SyntaxKind :: L_CURLY } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [-=] => { $ crate :: SyntaxKind :: MINUSEQ } ; [!=] => { $ crate :: SyntaxKind :: NEQ } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [%=] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [|] => { $ crate :: SyntaxKind :: PIPE } ; [||] => { $ crate :: SyntaxKind :: PIPE2 } ; [|=] => { $ crate :: SyntaxKind :: PIPEEQ } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [+=] => { $ crate :: SyntaxKind :: PLUSEQ } ; [#] => { $ crate :: SyntaxKind :: POUND } ; [?] => { $ crate :: SyntaxKind :: QUESTION } ; [>] => { $ crate :: SyntaxKind :: R_ANGLE } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; ['}'] => { $ crate :: SyntaxKind :: R_CURLY } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [<<] => { $ crate :: SyntaxKind :: SHL } ; [<<=] => { $ crate :: SyntaxKind :: SHLEQ } ; [>>] => { $ crate :: SyntaxKind :: SHR } ; [>>=] => { $ crate :: SyntaxKind :: SHREQ } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [/=] => { $ crate :: SyntaxKind :: SLASHEQ } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [*=] => { $ crate :: SyntaxKind :: STAREQ } ; [->] => { $ crate :: SyntaxKind :: THIN_ARROW } ; [~] => { $ crate :: SyntaxKind :: TILDE } ; [_] => { $ crate :: SyntaxKind :: UNDERSCORE } ; [Self] => { $ crate :: SyntaxKind :: SELF_TYPE_KW } ; [abstract] => { $ crate :: SyntaxKind :: ABSTRACT_KW } ; [as] => { $ crate :: SyntaxKind :: AS_KW } ; [async] => { $ crate :: SyntaxKind :: ASYNC_KW } ; [await] => { $ crate :: SyntaxKind :: AWAIT_KW } ; [become] => { $ crate :: SyntaxKind :: BECOME_KW } ; [box] => { $ crate :: SyntaxKind :: BOX_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [const] => { $ crate :: SyntaxKind :: CONST_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [crate] => { $ crate :: SyntaxKind :: CRATE_KW } ; [do] => { $ crate :: SyntaxKind :: DO_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [enum] => { $ crate :: SyntaxKind :: ENUM_KW } ; [extern] => { $ crate :: SyntaxKind :: EXTERN_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [final] => { $ crate :: SyntaxKind :: FINAL_KW } ; [fn] => { $ crate :: SyntaxKind :: FN_KW } ; [for] => { $ crate :: SyntaxKind :: FOR_KW } ; [gen] => { $ crate :: SyntaxKind :: GEN_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [impl] => { $ crate :: SyntaxKind :: IMPL_KW } ; [in] => { $ crate :: SyntaxKind :: IN_KW } ; [let] => { $ crate :: SyntaxKind :: LET_KW } ; [loop] => { $ crate :: SyntaxKind :: LOOP_KW } ; [macro] => { $ crate :: SyntaxKind :: MACRO_KW } ; [match] => { $ crate :: SyntaxKind :: MATCH_KW } ; [mod] => { $ crate :: SyntaxKind :: MOD_KW } ; [move] => { $ crate :: SyntaxKind :: MOVE_KW } ; [mut] => { $ crate :: SyntaxKind :: MUT_KW } ; [override] => { $ crate :: SyntaxKind :: OVERRIDE_KW } ; [priv] => { $ crate :: SyntaxKind :: PRIV_KW } ; [pub] => { $ crate :: SyntaxKind :: PUB_KW } ; [ref] => { $ crate :: SyntaxKind :: REF_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [self] => { $ crate :: SyntaxKind :: SELF_KW } ; [static] => { $ crate :: SyntaxKind :: STATIC_KW } ; [struct] => { $ crate :: SyntaxKind :: STRUCT_KW } ; [super] => { $ crate :: SyntaxKind :: SUPER_KW } ; [trait] => { $ crate :: SyntaxKind :: TRAIT_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [try] => { $ crate :: SyntaxKind :: TRY_KW } ; [type] => { $ crate :: SyntaxKind :: TYPE_KW } ; [typeof] => { $ crate :: SyntaxKind :: TYPEOF_KW } ; [unsafe] => { $ crate :: SyntaxKind :: UNSAFE_KW } ; [unsized] => { $ crate :: SyntaxKind :: UNSIZED_KW } ; [use] => { $ crate :: SyntaxKind :: USE_KW } ; [virtual] => { $ crate :: SyntaxKind :: VIRTUAL_KW } ; [where] => { $ crate :: SyntaxKind :: WHERE_KW } ; [while] => { $ crate :: SyntaxKind :: WHILE_KW } ; [yield] => { $ crate :: SyntaxKind :: YIELD_KW } ; [builtin] => { $ crate :: SyntaxKind :: BUILTIN_KW } ; [offset_of] => { $ crate :: SyntaxKind :: OFFSET_OF_KW } ; [format_args] => { $ crate :: SyntaxKind :: FORMAT_ARGS_KW } ; [asm] => { $ crate :: SyntaxKind :: ASM_KW } ; [macro_rules] => { $ crate :: SyntaxKind :: MACRO_RULES_KW } ; [union] => { $ crate :: SyntaxKind :: UNION_KW } ; [default] => { $ crate :: SyntaxKind :: DEFAULT_KW } ; [raw] => { $ crate :: SyntaxKind :: RAW_KW } ; [dyn] => { $ crate :: SyntaxKind :: DYN_KW } ; [auto] => { $ crate :: SyntaxKind :: AUTO_KW } ; [yeet] => { $ crate :: SyntaxKind :: YEET_KW } ; [lifetime_ident] => { $ crate :: SyntaxKind :: LIFETIME_IDENT } ; [int_number] => { $ crate :: SyntaxKind :: INT_NUMBER } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [string] => { $ crate :: SyntaxKind :: STRING } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; }
