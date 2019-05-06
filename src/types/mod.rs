use num_derive::FromPrimitive;
use wasm_bindgen::prelude::*;

pub mod textspan;

#[derive(FromPrimitive)]
pub enum SyntaxKind {
    Unknown = 0,
    EndOfFileToken,
    SingleLineCommentTrivia,
    MultiLineCommentTrivia,
    NewLineTrivia,
    WhitespaceTrivia,
    // We detect and preserve #! on the first line
    ShebangTrivia,
    // We detect and provide better error recovery when we encounter a git merge marker.  This
    // allows us to edit files with git-conflict markers in them in a much more pleasant manner.
    ConflictMarkerTrivia,
    // Literals
    NumericLiteral,
    BigIntLiteral,
    StringLiteral,
    JsxText,
    JsxTextAllWhiteSpaces,
    RegularExpressionLiteral,
    NoSubstitutionTemplateLiteral,
    // Pseudo-literals
    TemplateHead,
    TemplateMiddle,
    TemplateTail,
    // Punctuation
    OpenBraceToken,
    CloseBraceToken,
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    DotToken,
    DotDotDotToken,
    SemicolonToken,
    CommaToken,
    LessThanToken,
    LessThanSlashToken,
    GreaterThanToken,
    LessThanEqualsToken,
    GreaterThanEqualsToken,
    EqualsEqualsToken,
    ExclamationEqualsToken,
    EqualsEqualsEqualsToken,
    ExclamationEqualsEqualsToken,
    EqualsGreaterThanToken,
    PlusToken,
    MinusToken,
    AsteriskToken,
    AsteriskAsteriskToken,
    SlashToken,
    PercentToken,
    PlusPlusToken,
    MinusMinusToken,
    LessThanLessThanToken,
    GreaterThanGreaterThanToken,
    GreaterThanGreaterThanGreaterThanToken,
    AmpersandToken,
    BarToken,
    CaretToken,
    ExclamationToken,
    TildeToken,
    AmpersandAmpersandToken,
    BarBarToken,
    QuestionToken,
    ColonToken,
    AtToken,
    // Assignments
    EqualsToken,
    PlusEqualsToken,
    MinusEqualsToken,
    AsteriskEqualsToken,
    AsteriskAsteriskEqualsToken,
    SlashEqualsToken,
    PercentEqualsToken,
    LessThanLessThanEqualsToken,
    GreaterThanGreaterThanEqualsToken,
    GreaterThanGreaterThanGreaterThanEqualsToken,
    AmpersandEqualsToken,
    BarEqualsToken,
    CaretEqualsToken,
    // Identifiers
    Identifier,
    // Reserved words
    BreakKeyword,
    CaseKeyword,
    CatchKeyword,
    ClassKeyword,
    ConstKeyword,
    ContinueKeyword,
    DebuggerKeyword,
    DefaultKeyword,
    DeleteKeyword,
    DoKeyword,
    ElseKeyword,
    EnumKeyword,
    ExportKeyword,
    ExtendsKeyword,
    FalseKeyword,
    FinallyKeyword,
    ForKeyword,
    FunctionKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    InstanceOfKeyword,
    NewKeyword,
    NullKeyword,
    ReturnKeyword,
    SuperKeyword,
    SwitchKeyword,
    ThisKeyword,
    ThrowKeyword,
    TrueKeyword,
    TryKeyword,
    TypeOfKeyword,
    VarKeyword,
    VoidKeyword,
    WhileKeyword,
    WithKeyword,
    // Strict mode reserved words
    ImplementsKeyword,
    InterfaceKeyword,
    LetKeyword,
    PackageKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    StaticKeyword,
    YieldKeyword,
    // Contextual keywords
    AbstractKeyword,
    AsKeyword,
    AnyKeyword,
    AsyncKeyword,
    AwaitKeyword,
    BooleanKeyword,
    ConstructorKeyword,
    DeclareKeyword,
    GetKeyword,
    InferKeyword,
    IsKeyword,
    KeyOfKeyword,
    ModuleKeyword,
    NamespaceKeyword,
    NeverKeyword,
    ReadonlyKeyword,
    RequireKeyword,
    NumberKeyword,
    ObjectKeyword,
    SetKeyword,
    StringKeyword,
    SymbolKeyword,
    TypeKeyword,
    UndefinedKeyword,
    UniqueKeyword,
    UnknownKeyword,
    FromKeyword,
    GlobalKeyword,
    BigIntKeyword,
    OfKeyword, // LastKeyword and LastToken and LastContextualKeyword

    // Parse tree nodes

    // Names
    QualifiedName,
    ComputedPropertyName,
    // Signature elements
    TypeParameter,
    Parameter,
    Decorator,
    // TypeMember
    PropertySignature,
    PropertyDeclaration,
    MethodSignature,
    MethodDeclaration,
    Constructor,
    GetAccessor,
    SetAccessor,
    CallSignature,
    ConstructSignature,
    IndexSignature,
    // Type
    TypePredicate,
    TypeReference,
    FunctionType,
    ConstructorType,
    TypeQuery,
    TypeLiteral,
    ArrayType,
    TupleType,
    OptionalType,
    RestType,
    UnionType,
    IntersectionType,
    ConditionalType,
    InferType,
    ParenthesizedType,
    ThisType,
    TypeOperator,
    IndexedAccessType,
    MappedType,
    LiteralType,
    ImportType,
    // Binding patterns
    ObjectBindingPattern,
    ArrayBindingPattern,
    BindingElement,
    // Expression
    ArrayLiteralExpression,
    ObjectLiteralExpression,
    PropertyAccessExpression,
    ElementAccessExpression,
    CallExpression,
    NewExpression,
    TaggedTemplateExpression,
    TypeAssertionExpression,
    ParenthesizedExpression,
    FunctionExpression,
    ArrowFunction,
    DeleteExpression,
    TypeOfExpression,
    VoidExpression,
    AwaitExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    BinaryExpression,
    ConditionalExpression,
    TemplateExpression,
    YieldExpression,
    SpreadElement,
    ClassExpression,
    OmittedExpression,
    ExpressionWithTypeArguments,
    AsExpression,
    NonNullExpression,
    MetaProperty,
    SyntheticExpression,

    // Misc
    TemplateSpan,
    SemicolonClassElement,
    // Element
    Block,
    VariableStatement,
    EmptyStatement,
    ExpressionStatement,
    IfStatement,
    DoStatement,
    WhileStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,
    WithStatement,
    SwitchStatement,
    LabeledStatement,
    ThrowStatement,
    TryStatement,
    DebuggerStatement,
    VariableDeclaration,
    VariableDeclarationList,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    ModuleDeclaration,
    ModuleBlock,
    CaseBlock,
    NamespaceExportDeclaration,
    ImportEqualsDeclaration,
    ImportDeclaration,
    ImportClause,
    NamespaceImport,
    NamedImports,
    ImportSpecifier,
    ExportAssignment,
    ExportDeclaration,
    NamedExports,
    ExportSpecifier,
    MissingDeclaration,

    // Module references
    ExternalModuleReference,

    // JSX
    JsxElement,
    JsxSelfClosingElement,
    JsxOpeningElement,
    JsxClosingElement,
    JsxFragment,
    JsxOpeningFragment,
    JsxClosingFragment,
    JsxAttribute,
    JsxAttributes,
    JsxSpreadAttribute,
    JsxExpression,

    // Clauses
    CaseClause,
    DefaultClause,
    HeritageClause,
    CatchClause,

    // Property assignments
    PropertyAssignment,
    ShorthandPropertyAssignment,
    SpreadAssignment,

    // Enum
    EnumMember,
    // Unparsed
    UnparsedPrologue,
    UnparsedPrepend,
    UnparsedText,
    UnparsedInternalText,
    UnparsedSyntheticReference,

    // Top-level nodes
    SourceFile,
    Bundle,
    UnparsedSource,
    InputFiles,

    // JSDoc nodes
    JSDocTypeExpression,
    // The * type
    JSDocAllType,
    // The ? type
    JSDocUnknownType,
    JSDocNullableType,
    JSDocNonNullableType,
    JSDocOptionalType,
    JSDocFunctionType,
    JSDocVariadicType,
    JSDocComment,
    JSDocTypeLiteral,
    JSDocSignature,
    JSDocTag,
    JSDocAugmentsTag,
    JSDocClassTag,
    JSDocCallbackTag,
    JSDocEnumTag,
    JSDocParameterTag,
    JSDocReturnTag,
    JSDocThisTag,
    JSDocTypeTag,
    JSDocTemplateTag,
    JSDocTypedefTag,
    JSDocPropertyTag,

    // Synthesized list
    SyntaxList,

    // Transformation nodes
    NotEmittedStatement,
    PartiallyEmittedExpression,
    CommaListExpression,
    MergeDeclarationMarker,
    EndOfDeclarationMarker,

    // Enum value count
    Count,
}

#[derive(FromPrimitive, PartialEq, PartialOrd)]
pub enum CharacterCodes {
    NullCharacter = 0,
    MaxAsciiCharacter = 0x7F,

    LineFeed = 0x0A,       // \n
    CarriageReturn = 0x0D, // \r
    LineSeparator = 0x2028,
    ParagraphSeparator = 0x2029,
    NextLine = 0x0085,

    // Unicode 3.0 space characters
    Space = 0x0020,            // " "
    NonBreakingSpace = 0x00A0, //
    EnQuad = 0x2000,
    EmQuad = 0x2001,
    EnSpace = 0x2002,
    EmSpace = 0x2003,
    ThreePerEmSpace = 0x2004,
    FourPerEmSpace = 0x2005,
    SixPerEmSpace = 0x2006,
    FigureSpace = 0x2007,
    PunctuationSpace = 0x2008,
    ThinSpace = 0x2009,
    HairSpace = 0x200A,
    ZeroWidthSpace = 0x200B,
    NarrowNoBreakSpace = 0x202F,
    IdeographicSpace = 0x3000,
    MathematicalSpace = 0x205F,
    Ogham = 0x1680,

    Underscore = 0x5F,
    Dollar = 0x24,

    _0 = 0x30,
    _1 = 0x31,
    _2 = 0x32,
    _3 = 0x33,
    _4 = 0x34,
    _5 = 0x35,
    _6 = 0x36,
    _7 = 0x37,
    _8 = 0x38,
    _9 = 0x39,

    LowercaseA = 0x61,
    LowercaseB = 0x62,
    LowercaseC = 0x63,
    LowercaseD = 0x64,
    LowercaseE = 0x65,
    LowercaseF = 0x66,
    LowercaseG = 0x67,
    LowercaseH = 0x68,
    LowercaseI = 0x69,
    LowercaseJ = 0x6A,
    LowercaseK = 0x6B,
    LowercaseL = 0x6C,
    LowercaseM = 0x6D,
    LowercaseN = 0x6E,
    LowercaseO = 0x6F,
    LowercaseP = 0x70,
    LowercaseQ = 0x71,
    LowercaseR = 0x72,
    LowercaseS = 0x73,
    LowercaseT = 0x74,
    LowercaseU = 0x75,
    LowercaseV = 0x76,
    LowercaseW = 0x77,
    LowercaseX = 0x78,
    LowercaseY = 0x79,
    LowercaseZ = 0x7A,

    UppercaseA = 0x41,
    UppercaseB = 0x42,
    UppercaseC = 0x43,
    UppercaseD = 0x44,
    UppercaseE = 0x45,
    UppercaseF = 0x46,
    UppercaseG = 0x47,
    UppercaseH = 0x48,
    UppercaseI = 0x49,
    UppercaseJ = 0x4A,
    UppercaseK = 0x4B,
    UppercaseL = 0x4C,
    UppercaseM = 0x4D,
    UppercaseN = 0x4E,
    UppercaseO = 0x4F,
    UppercaseP = 0x50,
    UppercaseQ = 0x51,
    UppercaseR = 0x52,
    UppercaseS = 0x53,
    UppercaseT = 0x54,
    UppercaseU = 0x55,
    UppercaseV = 0x56,
    UppercaseW = 0x57,
    UppercaseX = 0x58,
    UppercaseY = 0x59,
    UppercaseZ = 0x5a,

    Ampersand = 0x26,    // &
    Asterisk = 0x2A,     // *
    At = 0x40,           // @
    Backslash = 0x5C,    // \
    Backtick = 0x60,     // `
    Bar = 0x7C,          // |
    Caret = 0x5E,        // ^
    CloseBrace = 0x7D,   // }
    CloseBracket = 0x5D, // ]
    CloseParen = 0x29,   // )
    Colon = 0x3A,        // :
    Comma = 0x2C,        // ,
    Dot = 0x2E,          // .
    DoubleQuote = 0x22,  // "
    Equals = 0x3D,       // =
    Exclamation = 0x21,  // !
    GreaterThan = 0x3E,  // >
    Hash = 0x23,         // #
    LessThan = 0x3C,     // <
    Minus = 0x2D,        // -
    OpenBrace = 0x7B,    // {
    OpenBracket = 0x5B,  // [
    OpenParen = 0x28,    // (
    Percent = 0x25,      // %
    Plus = 0x2B,         // +
    Question = 0x3F,     // ?
    Semicolon = 0x3B,    // ;
    SingleQuote = 0x27,  // '
    Slash = 0x2F,        // /
    Tilde = 0x7E,        // ~

    Backspace = 0x08, // \b
    FormFeed = 0x0C,  // \f
    ByteOrderMark = 0xFEFF,
    Tab = 0x09,         // \t
    VerticalTab = 0x0B, // \v
}

#[wasm_bindgen]
extern "C" {
    pub type PseudoBigIntJs;
    #[wasm_bindgen(method, getter = negative)]
    pub fn get_negative(this: &PseudoBigIntJs) -> bool;
    #[wasm_bindgen(method, getter = base10Value)]
    pub fn get_base10_value(this: &PseudoBigIntJs) -> String;
}

#[derive(Clone)]
pub(crate) struct PseudoBigInt {
    negative: bool,
    base10_value: String,
}

impl From<&PseudoBigIntJs> for PseudoBigInt {
    fn from(pseudo_big_int_js: &PseudoBigIntJs) -> PseudoBigInt {
        PseudoBigInt {
            negative: pseudo_big_int_js.get_negative(),
            base10_value: pseudo_big_int_js.get_base10_value(),
        }
    }
}

impl std::fmt::Display for PseudoBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let prefix = if self.negative && self.base10_value != "0" {
            "-"
        } else {
            ""
        };
        write!(f, "{}", prefix)?;
        write!(f, "{}", self.base10_value)
    }
}
