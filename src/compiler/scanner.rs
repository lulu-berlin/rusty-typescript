use crate::types::{CharacterCodes, SyntaxKind};
use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;

const ABSTRACT: &'static str = "abstract";
const ANY: &'static str = "any";
const AS: &'static str = "as";
const BIGINT: &'static str = "bigint";
const BOOLEAN: &'static str = "boolean";
const BREAK: &'static str = "break";
const CASE: &'static str = "case";
const CATCH: &'static str = "catch";
const CLASS: &'static str = "class";
const CONTINUE: &'static str = "continue";
const CONST: &'static str = "const";
const CONSTRUCTOR: &'static str = "constructor";
const DEBUGGER: &'static str = "debugger";
const DECLARE: &'static str = "declare";
const DEFAULT: &'static str = "default";
const DELETE: &'static str = "delete";
const DO: &'static str = "do";
const ELSE: &'static str = "else";
const ENUM: &'static str = "enum";
const EXPORT: &'static str = "export";
const EXTENDS: &'static str = "extends";
const FALSE: &'static str = "false";
const FINALLY: &'static str = "finally";
const FOR: &'static str = "for";
const FROM: &'static str = "from";
const FUNCTION: &'static str = "function";
const GET: &'static str = "get";
const IF: &'static str = "if";
const IMPLEMENTS: &'static str = "implements";
const IMPORT: &'static str = "import";
const IN: &'static str = "in";
const INFER: &'static str = "infer";
const INSTANCEOF: &'static str = "instanceof";
const INTERFACE: &'static str = "interface";
const IS: &'static str = "is";
const KEYOF: &'static str = "keyof";
const LET: &'static str = "let";
const MODULE: &'static str = "module";
const NAMESPACE: &'static str = "namespace";
const NEVER: &'static str = "never";
const NEW: &'static str = "new";
const NULL: &'static str = "null";
const NUMBER: &'static str = "number";
const OBJECT: &'static str = "object";
const PACKAGE: &'static str = "package";
const PRIVATE: &'static str = "private";
const PROTECTED: &'static str = "protected";
const PUBLIC: &'static str = "public";
const READONLY: &'static str = "readonly";
const REQUIRE: &'static str = "require";
const GLOBAL: &'static str = "global";
const RETURN: &'static str = "return";
const SET: &'static str = "set";
const STATIC: &'static str = "static";
const STRING: &'static str = "string";
const SUPER: &'static str = "super";
const SWITCH: &'static str = "switch";
const SYMBOL: &'static str = "symbol";
const THIS: &'static str = "this";
const THROW: &'static str = "throw";
const TRUE: &'static str = "true";
const TRY: &'static str = "try";
const TYPE: &'static str = "type";
const TYPEOF: &'static str = "typeof";
const UNDEFINED: &'static str = "undefined";
const UNIQUE: &'static str = "unique";
const UNKNOWN: &'static str = "unknown";
const VAR: &'static str = "var";
const VOID: &'static str = "void";
const WHILE: &'static str = "while";
const WITH: &'static str = "with";
const YIELD: &'static str = "yield";
const ASYNC: &'static str = "async";
const AWAIT: &'static str = "await";
const OF: &'static str = "of";
const OPEN_BRACE_TOKEN: &'static str = "{";
const CLOSE_BRACE_TOKEN: &'static str = "}";
const OPEN_PAREN_TOKEN: &'static str = "(";
const CLOSE_PAREN_TOKEN: &'static str = ")";
const OPEN_BRACKET_TOKEN: &'static str = "[";
const CLOSE_BRACKET_TOKEN: &'static str = "]";
const DOT_TOKEN: &'static str = ".";
const DOT_DOT_DOT_TOKEN: &'static str = "...";
const SEMICOLON_TOKEN: &'static str = ";";
const COMMA_TOKEN: &'static str = ",";
const LESS_THAN_TOKEN: &'static str = "<";
const GREATER_THAN_TOKEN: &'static str = ">";
const LESS_THAN_EQUALS_TOKEN: &'static str = "<=";
const GREATER_THAN_EQUALS_TOKEN: &'static str = ">=";
const EQUALS_EQUALS_TOKEN: &'static str = "==";
const EXCLAMATION_EQUALS_TOKEN: &'static str = "!=";
const EQUALS_EQUALS_EQUALS_TOKEN: &'static str = "===";
const EXCLAMATION_EQUALS_EQUALS_TOKEN: &'static str = "!==";
const EQUALS_GREATER_THAN_TOKEN: &'static str = "=>";
const PLUS_TOKEN: &'static str = "+";
const MINUS_TOKEN: &'static str = "-";
const ASTERISK_ASTERISK_TOKEN: &'static str = "**";
const ASTERISK_TOKEN: &'static str = "*";
const SLASH_TOKEN: &'static str = "/";
const PERCENT_TOKEN: &'static str = "%";
const PLUS_PLUS_TOKEN: &'static str = "++";
const MINUS_MINUS_TOKEN: &'static str = "--";
const LESS_THAN_LESS_THAN_TOKEN: &'static str = "<<";
const LESS_THAN_SLASH_TOKEN: &'static str = "</";
const GREATER_THAN_GREATER_THAN_TOKEN: &'static str = ">>";
const GREATER_THAN_GREATER_THAN_GREATER_THAN_TOKEN: &'static str = ">>>";
const AMPERSAND_TOKEN: &'static str = "&";
const BAR_TOKEN: &'static str = "|";
const CARET_TOKEN: &'static str = "^";
const EXCLAMATION_TOKEN: &'static str = "!";
const TILDE_TOKEN: &'static str = "~";
const AMPERSAND_AMPERSAND_TOKEN: &'static str = "&&";
const BAR_BAR_TOKEN: &'static str = "||";
const QUESTION_TOKEN: &'static str = "?";
const COLON_TOKEN: &'static str = ":";
const EQUALS_TOKEN: &'static str = "=";
const PLUS_EQUALS_TOKEN: &'static str = "+=";
const MINUS_EQUALS_TOKEN: &'static str = "-=";
const ASTERISK_EQUALS_TOKEN: &'static str = "*=";
const ASTERISK_ASTERISK_EQUALS_TOKEN: &'static str = "**=";
const SLASH_EQUALS_TOKEN: &'static str = "/=";
const PERCENT_EQUALS_TOKEN: &'static str = "%=";
const LESS_THAN_LESS_THAN_EQUALS_TOKEN: &'static str = "<<=";
const GREATER_THAN_GREATER_THAN_EQUALS_TOKEN: &'static str = ">>=";
const GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUALS_TOKEN: &'static str = ">>>=";
const AMPERSAND_EQUALS_TOKEN: &'static str = "&=";
const BAR_EQUALS_TOKEN: &'static str = "|=";
const CARET_EQUALS_TOKEN: &'static str = "^=";
const AT_TOKEN: &'static str = "@";

#[wasm_bindgen]
pub fn token_to_string(t: u32) -> Option<String> {
    match FromPrimitive::from_u32(t) {
        Some(SyntaxKind::AbstractKeyword) => Some(ABSTRACT.into()),
        Some(SyntaxKind::AnyKeyword) => Some(ANY.into()),
        Some(SyntaxKind::AsKeyword) => Some(AS.into()),
        Some(SyntaxKind::BigIntKeyword) => Some(BIGINT.into()),
        Some(SyntaxKind::BooleanKeyword) => Some(BOOLEAN.into()),
        Some(SyntaxKind::BreakKeyword) => Some(BREAK.into()),
        Some(SyntaxKind::CaseKeyword) => Some(CASE.into()),
        Some(SyntaxKind::CatchKeyword) => Some(CATCH.into()),
        Some(SyntaxKind::ClassKeyword) => Some(CLASS.into()),
        Some(SyntaxKind::ContinueKeyword) => Some(CONTINUE.into()),
        Some(SyntaxKind::ConstKeyword) => Some(CONST.into()),
        Some(SyntaxKind::ConstructorKeyword) => Some(CONSTRUCTOR.into()),
        Some(SyntaxKind::DebuggerKeyword) => Some(DEBUGGER.into()),
        Some(SyntaxKind::DeclareKeyword) => Some(DECLARE.into()),
        Some(SyntaxKind::DefaultKeyword) => Some(DEFAULT.into()),
        Some(SyntaxKind::DeleteKeyword) => Some(DELETE.into()),
        Some(SyntaxKind::DoKeyword) => Some(DO.into()),
        Some(SyntaxKind::ElseKeyword) => Some(ELSE.into()),
        Some(SyntaxKind::EnumKeyword) => Some(ENUM.into()),
        Some(SyntaxKind::ExportKeyword) => Some(EXPORT.into()),
        Some(SyntaxKind::ExtendsKeyword) => Some(EXTENDS.into()),
        Some(SyntaxKind::FalseKeyword) => Some(FALSE.into()),
        Some(SyntaxKind::FinallyKeyword) => Some(FINALLY.into()),
        Some(SyntaxKind::ForKeyword) => Some(FOR.into()),
        Some(SyntaxKind::FromKeyword) => Some(FROM.into()),
        Some(SyntaxKind::FunctionKeyword) => Some(FUNCTION.into()),
        Some(SyntaxKind::GetKeyword) => Some(GET.into()),
        Some(SyntaxKind::IfKeyword) => Some(IF.into()),
        Some(SyntaxKind::ImplementsKeyword) => Some(IMPLEMENTS.into()),
        Some(SyntaxKind::ImportKeyword) => Some(IMPORT.into()),
        Some(SyntaxKind::InKeyword) => Some(IN.into()),
        Some(SyntaxKind::InferKeyword) => Some(INFER.into()),
        Some(SyntaxKind::InstanceOfKeyword) => Some(INSTANCEOF.into()),
        Some(SyntaxKind::InterfaceKeyword) => Some(INTERFACE.into()),
        Some(SyntaxKind::IsKeyword) => Some(IS.into()),
        Some(SyntaxKind::KeyOfKeyword) => Some(KEYOF.into()),
        Some(SyntaxKind::LetKeyword) => Some(LET.into()),
        Some(SyntaxKind::ModuleKeyword) => Some(MODULE.into()),
        Some(SyntaxKind::NamespaceKeyword) => Some(NAMESPACE.into()),
        Some(SyntaxKind::NeverKeyword) => Some(NEVER.into()),
        Some(SyntaxKind::NewKeyword) => Some(NEW.into()),
        Some(SyntaxKind::NullKeyword) => Some(NULL.into()),
        Some(SyntaxKind::NumberKeyword) => Some(NUMBER.into()),
        Some(SyntaxKind::ObjectKeyword) => Some(OBJECT.into()),
        Some(SyntaxKind::PackageKeyword) => Some(PACKAGE.into()),
        Some(SyntaxKind::PrivateKeyword) => Some(PRIVATE.into()),
        Some(SyntaxKind::ProtectedKeyword) => Some(PROTECTED.into()),
        Some(SyntaxKind::PublicKeyword) => Some(PUBLIC.into()),
        Some(SyntaxKind::ReadonlyKeyword) => Some(READONLY.into()),
        Some(SyntaxKind::RequireKeyword) => Some(REQUIRE.into()),
        Some(SyntaxKind::GlobalKeyword) => Some(GLOBAL.into()),
        Some(SyntaxKind::ReturnKeyword) => Some(RETURN.into()),
        Some(SyntaxKind::SetKeyword) => Some(SET.into()),
        Some(SyntaxKind::StaticKeyword) => Some(STATIC.into()),
        Some(SyntaxKind::StringKeyword) => Some(STRING.into()),
        Some(SyntaxKind::SuperKeyword) => Some(SUPER.into()),
        Some(SyntaxKind::SwitchKeyword) => Some(SWITCH.into()),
        Some(SyntaxKind::SymbolKeyword) => Some(SYMBOL.into()),
        Some(SyntaxKind::ThisKeyword) => Some(THIS.into()),
        Some(SyntaxKind::ThrowKeyword) => Some(THROW.into()),
        Some(SyntaxKind::TrueKeyword) => Some(TRUE.into()),
        Some(SyntaxKind::TryKeyword) => Some(TRY.into()),
        Some(SyntaxKind::TypeKeyword) => Some(TYPE.into()),
        Some(SyntaxKind::TypeOfKeyword) => Some(TYPEOF.into()),
        Some(SyntaxKind::UndefinedKeyword) => Some(UNDEFINED.into()),
        Some(SyntaxKind::UniqueKeyword) => Some(UNIQUE.into()),
        Some(SyntaxKind::UnknownKeyword) => Some(UNKNOWN.into()),
        Some(SyntaxKind::VarKeyword) => Some(VAR.into()),
        Some(SyntaxKind::VoidKeyword) => Some(VOID.into()),
        Some(SyntaxKind::WhileKeyword) => Some(WHILE.into()),
        Some(SyntaxKind::WithKeyword) => Some(WITH.into()),
        Some(SyntaxKind::YieldKeyword) => Some(YIELD.into()),
        Some(SyntaxKind::AsyncKeyword) => Some(ASYNC.into()),
        Some(SyntaxKind::AwaitKeyword) => Some(AWAIT.into()),
        Some(SyntaxKind::OfKeyword) => Some(OF.into()),
        Some(SyntaxKind::OpenBraceToken) => Some(OPEN_BRACE_TOKEN.into()),
        Some(SyntaxKind::CloseBraceToken) => Some(CLOSE_BRACE_TOKEN.into()),
        Some(SyntaxKind::OpenParenToken) => Some(OPEN_PAREN_TOKEN.into()),
        Some(SyntaxKind::CloseParenToken) => Some(CLOSE_PAREN_TOKEN.into()),
        Some(SyntaxKind::OpenBracketToken) => Some(OPEN_BRACKET_TOKEN.into()),
        Some(SyntaxKind::CloseBracketToken) => Some(CLOSE_BRACKET_TOKEN.into()),
        Some(SyntaxKind::DotToken) => Some(DOT_TOKEN.into()),
        Some(SyntaxKind::DotDotDotToken) => Some(DOT_DOT_DOT_TOKEN.into()),
        Some(SyntaxKind::SemicolonToken) => Some(SEMICOLON_TOKEN.into()),
        Some(SyntaxKind::CommaToken) => Some(COMMA_TOKEN.into()),
        Some(SyntaxKind::LessThanToken) => Some(LESS_THAN_TOKEN.into()),
        Some(SyntaxKind::GreaterThanToken) => Some(GREATER_THAN_TOKEN.into()),
        Some(SyntaxKind::LessThanEqualsToken) => Some(LESS_THAN_EQUALS_TOKEN.into()),
        Some(SyntaxKind::GreaterThanEqualsToken) => Some(GREATER_THAN_EQUALS_TOKEN.into()),
        Some(SyntaxKind::EqualsEqualsToken) => Some(EQUALS_EQUALS_TOKEN.into()),
        Some(SyntaxKind::ExclamationEqualsToken) => Some(EXCLAMATION_EQUALS_TOKEN.into()),
        Some(SyntaxKind::EqualsEqualsEqualsToken) => Some(EQUALS_EQUALS_EQUALS_TOKEN.into()),
        Some(SyntaxKind::ExclamationEqualsEqualsToken) => {
            Some(EXCLAMATION_EQUALS_EQUALS_TOKEN.into())
        }
        Some(SyntaxKind::EqualsGreaterThanToken) => Some(EQUALS_GREATER_THAN_TOKEN.into()),
        Some(SyntaxKind::PlusToken) => Some(PLUS_TOKEN.into()),
        Some(SyntaxKind::MinusToken) => Some(MINUS_TOKEN.into()),
        Some(SyntaxKind::AsteriskAsteriskToken) => Some(ASTERISK_ASTERISK_TOKEN.into()),
        Some(SyntaxKind::AsteriskToken) => Some(ASTERISK_TOKEN.into()),
        Some(SyntaxKind::SlashToken) => Some(SLASH_TOKEN.into()),
        Some(SyntaxKind::PercentToken) => Some(PERCENT_TOKEN.into()),
        Some(SyntaxKind::PlusPlusToken) => Some(PLUS_PLUS_TOKEN.into()),
        Some(SyntaxKind::MinusMinusToken) => Some(MINUS_MINUS_TOKEN.into()),
        Some(SyntaxKind::LessThanLessThanToken) => Some(LESS_THAN_LESS_THAN_TOKEN.into()),
        Some(SyntaxKind::LessThanSlashToken) => Some(LESS_THAN_SLASH_TOKEN.into()),
        Some(SyntaxKind::GreaterThanGreaterThanToken) => {
            Some(GREATER_THAN_GREATER_THAN_TOKEN.into())
        }
        Some(SyntaxKind::GreaterThanGreaterThanGreaterThanToken) => {
            Some(GREATER_THAN_GREATER_THAN_GREATER_THAN_TOKEN.into())
        }
        Some(SyntaxKind::AmpersandToken) => Some(AMPERSAND_TOKEN.into()),
        Some(SyntaxKind::BarToken) => Some(BAR_TOKEN.into()),
        Some(SyntaxKind::CaretToken) => Some(CARET_TOKEN.into()),
        Some(SyntaxKind::ExclamationToken) => Some(EXCLAMATION_TOKEN.into()),
        Some(SyntaxKind::TildeToken) => Some(TILDE_TOKEN.into()),
        Some(SyntaxKind::AmpersandAmpersandToken) => Some(AMPERSAND_AMPERSAND_TOKEN.into()),
        Some(SyntaxKind::BarBarToken) => Some(BAR_BAR_TOKEN.into()),
        Some(SyntaxKind::QuestionToken) => Some(QUESTION_TOKEN.into()),
        Some(SyntaxKind::ColonToken) => Some(COLON_TOKEN.into()),
        Some(SyntaxKind::EqualsToken) => Some(EQUALS_TOKEN.into()),
        Some(SyntaxKind::PlusEqualsToken) => Some(PLUS_EQUALS_TOKEN.into()),
        Some(SyntaxKind::MinusEqualsToken) => Some(MINUS_EQUALS_TOKEN.into()),
        Some(SyntaxKind::AsteriskEqualsToken) => Some(ASTERISK_EQUALS_TOKEN.into()),
        Some(SyntaxKind::AsteriskAsteriskEqualsToken) => {
            Some(ASTERISK_ASTERISK_EQUALS_TOKEN.into())
        }
        Some(SyntaxKind::SlashEqualsToken) => Some(SLASH_EQUALS_TOKEN.into()),
        Some(SyntaxKind::PercentEqualsToken) => Some(PERCENT_EQUALS_TOKEN.into()),
        Some(SyntaxKind::LessThanLessThanEqualsToken) => {
            Some(LESS_THAN_LESS_THAN_EQUALS_TOKEN.into())
        }
        Some(SyntaxKind::GreaterThanGreaterThanEqualsToken) => {
            Some(GREATER_THAN_GREATER_THAN_EQUALS_TOKEN.into())
        }
        Some(SyntaxKind::GreaterThanGreaterThanGreaterThanEqualsToken) => {
            Some(GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUALS_TOKEN.into())
        }
        Some(SyntaxKind::AmpersandEqualsToken) => Some(AMPERSAND_EQUALS_TOKEN.into()),
        Some(SyntaxKind::BarEqualsToken) => Some(BAR_EQUALS_TOKEN.into()),
        Some(SyntaxKind::CaretEqualsToken) => Some(CARET_EQUALS_TOKEN.into()),
        Some(SyntaxKind::AtToken) => Some(AT_TOKEN.into()),
        _ => None,
    }
}

#[wasm_bindgen]
/* Does not include line breaks. For that, see is_white_space_like(). */
pub fn is_white_space_single_line(ch: u32) -> bool {
    // Note: NextLine is in the Zs space, and should be considered to be a whitespace.
    // It is explicitly not a line-break as it isn't in the exact set specified by EcmaScript.

    FromPrimitive::from_u32(ch)
        .map(|charcode: CharacterCodes| {
            charcode == CharacterCodes::Space
                || charcode == CharacterCodes::Tab
                || charcode == CharacterCodes::VerticalTab
                || charcode == CharacterCodes::FormFeed
                || charcode == CharacterCodes::NonBreakingSpace
                || charcode == CharacterCodes::NextLine
                || charcode == CharacterCodes::Ogham
                || charcode >= CharacterCodes::EnQuad && charcode <= CharacterCodes::ZeroWidthSpace
                || charcode == CharacterCodes::NarrowNoBreakSpace
                || charcode == CharacterCodes::MathematicalSpace
                || charcode == CharacterCodes::IdeographicSpace
                || charcode == CharacterCodes::IdeographicSpace
                || charcode == CharacterCodes::ByteOrderMark
        })
        .unwrap_or(false)
}

#[wasm_bindgen]
pub fn is_line_break(ch: u32) -> bool {
    // ES5 7.3:
    // The ECMAScript line terminator characters are listed in Table 3.
    //     Table 3: Line Terminator Characters
    //     Code Unit Value     Name                    Formal Name
    //     \u000A              Line Feed               <LF>
    //     \u000D              Carriage Return         <CR>
    //     \u2028              Line separator          <LS>
    //     \u2029              Paragraph separator     <PS>
    // Only the characters in Table 3 are treated as line terminators. Other new line or line
    // breaking characters are treated as white space but not as line terminators.

    FromPrimitive::from_u32(ch)
        .map(|charcode: CharacterCodes| {
            charcode == CharacterCodes::LineFeed
                || charcode == CharacterCodes::CarriageReturn
                || charcode == CharacterCodes::LineSeparator
                || charcode == CharacterCodes::ParagraphSeparator
        })
        .unwrap_or(false)
}

#[wasm_bindgen]
pub fn is_white_space_like(ch: u32) -> bool {
    is_white_space_single_line(ch) || is_line_break(ch)
}
