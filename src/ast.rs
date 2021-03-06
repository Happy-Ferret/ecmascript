//! This module contains type definitions for the Abstract Syntax elements
//! that make up the ECMAScript language.
//!
//! The types have been designed to be easily understandable and readable. That means
//! we do not explicitly disallow invalid syntax trees in favour of simplicity.
//! For example, the TaggedTemplate is only allowed to have a TemplateLiteral
//! expression as its quasi, but we do not enforce this in the type definition
//! for the sake of brevity.
//!
//! The macros `build_ast` and `match_ast` are meant to be the public API of this
//! module as they abstract away the types in such a way so that the user of the library
//! feels as if they are working with source text almost directly.

/// NullLiteral is the syntax element for `null`.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-null-literals)
#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral;

/// BooleanLiteral is the syntax element for `true` and `false`.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-boolean-literals)
pub type BooleanLiteral = bool;

/// NumberLiteral is the syntax element for numbers. The parser will convert the string
/// values into an f64 for the sake of simplicity.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-numeric-literals)
pub type NumberLiteral = f64;

/// StringLiteral is a syntax element with quotes (single or double).
/// eg. `'my string literal'` or `"my other string literal"`
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-literals-string-literals)
pub type StringLiteral = String;

/// Id is an identifier in the ecmascript language.
/// eg. `var foo = {};`
/// `foo` is the identifier.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-identifier-names).
pub type Id = String;

/// RegexLiteral is the syntax element of a regular expression.
/// eg. `/abc[123]/gi`
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-literals-regular-expression-literals)
#[derive(Debug, Clone, PartialEq)]
pub struct RegexLiteral {
    /// This is the text between the slashes.
    pub pattern: String,
    /// This is the text after the slashes. eg the `i` flag is the case insensitive flag.
    pub flags: String,
}

/// TemplateElement is any text between interpolated expressions inside a template literal.
/// eg. ``abc ${} \u{2028}``
/// "abc " and " \u{2028}" would be the TemplateElements for this template literal.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-template-literal-lexical-components)
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement {
    /// If the template element has any sort of escape sequences (eg. \u{2028})
    /// this will represent the evaluated result of that sequence.
    /// eg. if raw == "\u{41}", cooked = "A"
    pub cooked: String,
    /// This will store the exact string value, before being evaluted into the unicode
    /// code points.
    pub raw: String,
}

/// Expression is an enumeration of all possible expressions merged into one big enum.
/// This also includes language extensions, such as JSX.
///
/// This represents all possible computations that can be done in the ecmascript language.
///
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-ecmascript-language-expressions)
/// [Primary Expression](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-primary-expression)
/// [Left Hand Side Expressions](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-left-hand-side-expressions)
/// [Update Expressions](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-update-expressions)
/// [JSX Specification](https://facebook.github.io/jsx/)
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// The 'this' keyword is a primary expression.
    This,
    /// An identifier can also be a primary expression.
    IdReference(Id),
    /// This is all literals minus the regex literal and the template literal.
    Literal(ExpressionLiteral),
    /// This is an expression created with [] brackets.
    ArrayLiteral(Vec<Expression>),
    /// This is an expression created by using {} brackets.
    ObjectLiteral(Vec<Property>),
    /// A function expression is a function defined in an expression position.
    /// Arrow functions are one where the body is a single statement that is an expression
    /// statement.
    Function {
        /// A function expression can be anonymous, where it has no name.
        id: Option<Id>,
        /// The formal parameters to a function.
        params: Vec<Id>,
        /// The body is a list of statements. This can include pragmas.
        body: Vec<Statement>,
        /// This is true if the function was defined with the `async` keyword before the
        /// `function` keyword.
        async: bool,
        /// This is true if there is a `*` character after the `function` keyword.
        generator: bool,
    },
    // Class,
    /// A regex literal can be used in expression position.
    /// eg (/asd/.test(123))
    RegexLiteral(RegexLiteral),
    /// A Template literal expression has many template elements with expressions littered
    /// between.
    ///
    /// When a template literal gets passed to the tagged template, it usually gets split into
    /// the quasis (the pieces between the interpolated expressions) as an array for the first
    /// argument, and the expressions get spread into the rest of the function call.
    ///
    /// For the sake of simplicity, we are not representing this in the AST.
    TemplateLiteral(Vec<TemplateLiteralElement>),
    /// A spread expression is an expression of the form `...(<expression>)`.
    Spread(Box<Expression>),
    /// A member expression is a property access expression.
    /// Eg. `obj.key` or `obj[computed_key]`
    Member {
        /// The lhs is the object we're trying to access.
        lhs: Box<Expression>,
        /// The rhs is the key we're trying to access. It can be computed, or a basic
        /// IdReference.
        rhs: Box<Expression>,
        /// This is true if the rhs was written with `[]` notation.
        computed: bool,
    },
    /// Super is the `super` keyword, similar to the `this` keyword.
    Super,
    /// This is the `new.target` expression that was introduced in ES2015. This
    /// tells you if the function was called with the `new` operator.
    MetaProperty,
    /// This is the `new MemberExpression` expression. It will construct the callee
    /// and return an object.
    New {
        /// The callee is the function we are trying to construct.
        callee: Box<Expression>,
        /// The arguments is a list of parameters to the function we're trying to construct.
        arguments: Vec<Expression>,
    },
    /// This is a regular function call, eg. `myFunction(expr1, expr2)`
    Call {
        /// The callee is the function we're trying to call. It may be an IIFE (immediately
        /// invoked function expression) or any other dynamic function.
        callee: Box<Expression>,
        /// The list of parameters to pass to the function.
        arguments: Vec<Expression>,
    },
    /// This is an expression where we pass the elements of the template literal to the
    /// tag function.
    ///
    /// eg.
    /// ```javascript
    /// function tag(stringParts, expr1, expr2) {
    ///
    /// }
    /// tag`123 ${}`
    /// ```
    TaggedTemplate {
        /// This is the function we're trying to pass the template elements to.
        tag: Box<Expression>,
        /// The only expression that is valid for the quasi, is another TemplateLiteral
        /// expression. In the interest of simplicity, we are not going to disallow this in the
        /// AST.
        quasi: Box<Expression>,
    },
    /// An update expression is either a postfix or prefix, increment or decrement, operator
    /// applied to an operand.
    Update {
        /// The operator is either ++ or --
        operator: UpdateOperator,
        /// The argument is another expression, eg. (++(a))
        argument: Box<Expression>,
        /// This tells you if the operator is in prefix or postfix position.
        prefix: bool,
    },
    /// A unary expression is a unary operator in prefix position to the operand.
    Unary {
        /// The operator is one that can only take a single operand.
        operator: UnaryOperator,
        /// The expression is the operand that is passed to the operator.
        argument: Box<Expression>,
    },
    /// The binary expression is one of the form (lhs operand rhs).
    Binary {
        /// The operand that is infixed between the operands.
        operator: BinaryOperator,
        /// The left hand side.
        lhs: Box<Expression>,
        /// The right hand side.
        rhs: Box<Expression>,
    },
    /// The ternary operator. This is of the form (test ? alternate : consequent)
    /// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-conditional-operator)
    Conditional {
        /// The expression before the ?. This must evaluate to a truthy or falsy value.
        test: Box<Expression>,
        /// The expression returned if the test expression is truthy.
        alternate: Box<Expression>,
        /// The expression returned if the test expression is falsy.
        consequent: Box<Expression>,
    },
    /// An assignment operator is one of the form (lhs assigned rhs). This changes the left hand
    /// side of the expression by applying an operator to the right hand side and the left hand
    /// side to get the new value of the left hand side.
    Assignment {
        /// The operator that is between the operands. This is slightly different to the binary
        /// expression, as it changes the LHS. The binary operators will return a new value
        /// instead of changing the left hand side.
        operator: AssignmentOperator,
        /// The expression that gets changed in some way. eg. (id = some_new_value)
        lhs: Box<Expression>,
        /// The expression that changes the lhs.
        rhs: Box<Expression>,
    },
    /// The yield expression that is only valid inside a generator function.
    /// It is a syntax error if there is a yield expression in the body of a non generator
    /// function.
    Yield {
        /// The generator may yield an expression to the caller, while requesting the caller to
        /// give back another value.
        argument: Option<Box<Expression>>,
        /// If the argument is another generator function, they must delegate all their yields to
        /// until the delegate generator completes.
        delegate: bool, // yield *
    },
    /// This represents a comma expression, eg. (a, b). This will evaluate the first operand,
    /// throw it away, and return the second operand.
    ///
    /// For a list of operands, it will evaluate all operands, throw them away, and then
    /// finally return the last operand.
    ///
    /// This is mainly useful for side effects, eg. (console.log(expr), expr).
    /// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-comma-operator)
    Comma(Vec<Expression>),
    /// *NOTE*: This is an extension to the language proposed by facebook.
    /// The JsxElement is an inlined expression of the form:
    /// <name key={value}>
    /// The JsxElement must be matched by a closing element, or else it is a syntax error.
    JsxElement {
        /// The name of the element to construct.
        name: String,
        /// The key={value} pairs.
        attributes: Vec<JsxAttribute>,
        /// The child elements.
        children: Vec<Expression>,
    },
    ///*NOTE*: This is an extension to the language proposed by facebook.
    /// This is an anonymous JsxElement, used when you want to return an array of
    /// elements without actually wrapping things into an unneeded DOM element.
    JsxFragment(Vec<Expression>),
}

/// This represents the Literal production of the PrimaryExpression rule.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#prod-Literal)
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionLiteral {
    /// This is a wrapper around the null literal.
    NullLiteral(NullLiteral),
    /// This is a wrapper around the boolean literal.
    BooleanLiteral(BooleanLiteral),
    /// This is a wrapper around the number literal.
    NumberLiteral(NumberLiteral),
    /// This is a wrapper around the string literal.
    StringLiteral(StringLiteral),
}

/// An object property is a tuple of a key, value, and a tag representing what kind of
/// property it is.
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    /// The key can be a computed expression, or an id reference.
    pub key: Expression,
    /// The value can be any sort of expression.
    pub value: Expression,
    /// The kind tells us if this is a getter, setter, or basic initializer.
    pub kind: PropertyKind,
}

/// An object property can be a getter, setter, or basic initializer.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    /// This just means the value is initialized to the expression. This is the default.
    Init,
    /// This means the value is a function that gets called when you try to access
    /// the key in the object. This allows you to return a dynamic value at property
    /// access time.
    Get,
    /// This means the value is a function that gets called when you try to
    /// set the property in the object.
    Set,
}

/// A template literal element can either be the string between backticks and `${`
/// or the expression between `${` and `}`.
/// This is easier than trying to re-construct the order.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateLiteralElement {
    /// A TemplateElement is the strings between the interpolated expressions.
    TemplateElement(TemplateElement),
    /// The expressions that are interpolated into the final value of the string.
    Expression(Expression),
}

/// These operators take 1 operand, update the operands mathematical value in the background,
/// then return an updated version of the operand.
///
/// If the operator is in postfix position, it returns the old value of the operand.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOperator {
    /// This will add 1 to the mathematical value of the operand. eg (a++ or ++a)
    Increment,
    /// This will subtract 1 from the mathematical value of the operand eg. (a-- or --a)
    Decrement,
}

/// These operators take 1 operand, and are a prefix of the operand.
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-unary-operators)
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// Reverse the sign on the operand. This will do type coercion first.
    /// eg. (-1)
    Minus,
    /// Make the operand a positive number. This will do type coercion first.
    /// eg (+(-1) is 1)
    Plus,
    /// Logically reverse the operand. This will do type coercion first.
    /// eg. (!true is false)
    Not,
    /// Logcally reverse all the bits on the operand. This will do type coercion first.
    /// eg (~9 is -10) (the sign bit is also reversed)
    BitwiseNot,
    /// Check the internal type of the operand, and return a string that represents the type.
    /// eg (typeof {}) is 'object'
    Typeof,
    /// This operator will evaluate the operand, and then return undefined itself.
    /// This can be used for invoke a function epxression immediately for example.
    Void,
    /// This operator will remove a property from an object. It will return true when
    /// the property was successfully deleted, and false when it wasnt.
    Delete,
}

/// All the operators that have 2 arguments are merged into one big enum here for simplicity
/// sake.
///
/// - [Multiplicative Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-multiplicative-operators)
/// - [Additive Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-additive-operators)
/// - [Bitwise Shift Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-bitwise-shift-operators)
/// - [Relational Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-relational-operators)
/// - [Equality Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-equality-operators)
/// - [Bitwise Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-binary-bitwise-operators)
/// - [Logical Operators](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-binary-logical-operators)
/// - [Exponentiation Operator](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-exp-operator)
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// The double equal operator that does type coercion. (a == b)
    EqEq,
    /// The not equal operator that does type coercion. (a != b)
    NotEq,
    /// The triple equal operator that compares types first, then values second. (a === b)
    EqEqEq,
    /// The not equal operator that compares types first, then values second. (a !== b)
    NotEqEq,
    /// The less than operator. (a < b)
    Lt,
    /// The less than or equal to operator. (a <= b)
    Lte,
    /// The greater than operator. (a > b)
    Gt,
    /// The greater than or equal to operator. (a >= b)
    Gte,
    /// The bitwise shift left operator. (eg. -2 << 1 is -4)
    Shl,
    /// The bitwise shift right operator. (eg. -8 >> 1 is -4)
    Shr,
    /// The unsigned bitwise shift right operator. (eg. -8 >>> 1 is 2147483644)
    UnsignedShr,
    /// (a + b)
    Plus,
    /// (a - b)
    Minus,
    /// (a * b)
    Multiply,
    /// (a / b)
    Divide,
    /// The modulo, or remainder operator. (eg. 7 % 2 is 1)
    Mod,
    /// The bitwise or operator. This does a logical or for each bit of both operands.
    /// (eg. 10 | 5 is 15, 1010 | 0101 = 1111)
    BitwiseOr,
    /// The logical or operator. This works on boolean values rather than numbers.
    /// (eg true || false is true)
    Or,
    /// The bitwise xor operator. This works by performing a logical xor for each bit of
    /// both operands. (eg. 10 ^ 6 is 12) (1010 ^ 0110 = 1100)
    BitwiseXor,
    /// The bitwise and operator. This works by performing a logical and for each bit of
    /// both operands. (eg. 10 & 6 is 2) (1010 & 0110 = 0010)
    BitwiseAnd,
    /// The logical and operator. This works on boolean values instead of numbers.
    /// (eg true && false is false)
    And,
    /// The key existence operator. This checks if a key exists in an object.
    /// eg. 'foo' in {'bar': 'baz'} is false
    In,
    /// The instanceof operator. This checks if the right hand operand exists anywhere
    /// in the prototype chain of the left hand operand.
    InstanceOf,
    /// The expoentation operator. This raises the left hand operand to the power of
    /// the right hand side. (eg 2 ** 4 is 2*2*2*2 or 16)
    Exponentiation,
}

/// Assignment operators are ones that signify a chnage to the left hand side of the expression.
///
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-assignment-operators)
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    /// The basic assignment statement. This changes the left hand side to become a
    /// copy of the right hand side. (eg. a = 1)
    Eq,
    /// This is shorthand for `lhs = lhs + rhs`. (eg a += 5).
    PlusEq,
    /// This is shorthand for `lhs = lhs - rhs`. (eg a -= 5).
    MinusEq,
    /// This is shorthand for `lhs = lhs * rhs`. (eg a *= 5).
    MultiplyEq,
    /// This is shorthand for `lhs = lhs / rhs`. (eg a /= 5).
    DivideEq,
    /// This is shorthand for `lhs = lhs % rhs`. (eg a %= 5).
    /// This is useful when the remainder of a division is more important than the division
    /// itself.
    ModEq,
    /// This is shorthand for `lhs = lhs << rhs`. (eg a <<= 5).
    /// This is useful when you want to shift all the bits of a variable
    /// without storing a copy of the variable.
    ShlEq,
    /// This is shorthand for `lhs = lhs >> rhs`. (eg a >>= 5).
    /// This is useful when you want to shift all the bits of a variable
    /// without storing a copy of the variable.
    ShrEq,
    /// This is shorthand for `lhs = lhs >>> rhs`. (eg a >>>= 5).
    /// The difference is that this will not preserve the minus sign of a number, like
    /// the >>= operation would.
    UnsignedShrEq,
    /// This is shorthand for `lhs = lhs | rhs`. (eg a |= 5).
    BitwiseOrEq,
    /// This is shorthand for `lhs = lhs ^ rhs`. (eg a ^= 5).
    BitwiseXorEq,
    /// This is shorthand for `lhs = lhs & rhs`. (eg a &= 5).
    BitwiseAndEq,
}

/// A JSX attribute is either a simple `key={value}` attribute, or a
/// spread of an object containing multiple attributes.
///
/// [Reference](https://facebook.github.io/jsx/)
#[derive(Debug, Clone, PartialEq)]
pub enum JsxAttribute {
    /// Spread an objects key value pairs into the JSX object as well.
    JsxSpreadAttribute {
        /// The expression could be typed more strictly into an ID Reference or an inline
        /// object, but for the sake of simplicity we reference the larger enum.
        expression: Expression,
    },
    /// A single `key={value}` pair. The value is optional, and if missing it means
    /// the existence of the key is more important than the value of the key.
    JsxAttribute {
        /// The key of the attribute.
        name: String,
        /// The optional value. If it is None, then it means the value is a boolean true.
        /// The absence of a key can mean false.
        value: Option<Expression>,
    },
}

/// A statement is either a declaration (var, const, let, function, export) or an
/// instruction to the interpreter to evaluate an expression.
/// For the sake of simplicity, declarations will get merged into this struct as well.
///
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-ecmascript-language-statements-and-declarations)
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {}

/// This is the main entry point to the syntax tree. A program is a list of statements,
/// and statements include declarations.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// This represents how the source is parsed. A module is parsed in strict mode, which
    /// disallows things in the parser level earlier on.
    pub source_type: SourceType,
    /// The list of statements or declarations made by the source text.
    pub body: Vec<Statement>,
}

/// This enum represents whether or not the source code contains an ECMAScript module.
/// An ECMAScript module can have import and export declarations in it, and has some
/// other subtle behaviour differences.
///
/// [Reference](https://www.ecma-international.org/ecma-262/9.0/index.html#sec-ecmascript-language-scripts-and-modules)
#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    /// The source text has no import or export declarations.
    Script,
    /// The source text has import or export declarations, and can be executed
    /// differently than a regular script.
    Module,
}
