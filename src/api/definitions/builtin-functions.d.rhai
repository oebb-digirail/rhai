/// Display any data to the standard output.
///
/// # Example
/// 
/// ```rhai
/// let answer = 42;
/// 
/// print(`The Answer is ${answer}`);
/// ```
fn print(data: ?);

/// Display any data to the standard output in debug format.
///
/// # Example
/// 
/// ```rhai
/// let answer = 42;
/// 
/// debug(answer);
/// ```
fn debug(data: ?);

/// Get the type of a value.
///
/// # Example
/// 
/// ```rhai
/// let x = "hello, world!";
/// 
/// print(x.type_of());     // prints "string"
/// ```
fn type_of(data: ?) -> String;

/// Create a function pointer to a named function.
///
/// If the specified name is not a valid function name, an error is raised.
/// 
/// # Example
/// 
/// ```rhai
/// let f = Fn("foo");      // function pointer to 'foo'
/// 
/// f.call(42);             // call: foo(42)
/// ```
fn Fn(fn_name: String) -> FnPtr;

/// Call a function pointed to by a function pointer,
/// passing following arguments to the function call.
///
/// If an appropriate function is not found, an error is raised.
/// 
/// # Example
/// 
/// ```rhai
/// let f = Fn("foo");      // function pointer to 'foo'
/// 
/// f.call(1, 2, 3);        // call: foo(1, 2, 3)
/// ```
fn call(fn_ptr: FnPtr, ...args: ?) -> ?;

/// Call a function pointed to by a function pointer, binding the `this` pointer
/// to the object of the method call, and passing on following arguments to the function call.
///
/// If an appropriate function is not found, an error is raised.
/// 
/// # Example
/// 
/// ```rhai
/// fn add(x) {
///     this + x
/// }
/// 
/// let f = Fn("add");      // function pointer to 'add'
/// 
/// let x = 41;
/// 
/// let r = x.call(f, 1);   // call: add(1) with 'this' = 'x'
/// 
/// print(r);               // prints 42
/// ```
fn call(obj: ?, fn_ptr: FnPtr, ...args: ?) -> ?;

/// Curry a number of arguments into a function pointer and return it as a new function pointer.
/// 
/// # Example
/// 
/// ```rhai
/// fn foo(x, y, z) {
///     x + y + z
/// }
/// 
/// let f = Fn("foo");
/// 
/// let g = f.curry(1, 2);  // curried arguments: 1, 2
/// 
/// g.call(3);              // call: foo(1, 2, 3)
/// ```
fn curry(fn_ptr: FnPtr, ...args: ?) -> FnPtr;

/// Return `true` if a script-defined function exists with a specified name and
/// number of parameters.
/// 
/// # Example
/// 
/// ```rhai
/// fn foo(x) { }
/// 
/// print(is_def_fn("foo", 1));     // prints true
/// print(is_def_fn("foo", 2));     // prints false
/// print(is_def_fn("foo", 0));     // prints false
/// print(is_def_fn("bar", 1));     // prints false
/// ```
fn is_def_fn(fn_name: String, num_params: int) -> bool;

/// Return `true` if a variable matching a specified name is defined.
/// 
/// # Example
/// 
/// ```rhai
/// let x = 42;
/// 
/// print(is_def_var("x"));         // prints true
/// print(is_def_var("foo"));       // prints false
/// 
/// {
///     let y = 1;
///     print(is_def_var("y"));     // prints true
/// }
/// 
/// print(is_def_var("y"));         // prints false
/// ```
fn is_def_var(var_name: String) -> bool;

/// Return `true` if the variable is shared.
/// 
/// # Example
/// 
/// ```rhai
/// let x = 42;
/// 
/// print(is_shared(x));        // prints false
/// 
/// let f = || x;               // capture 'x', making it shared
/// 
/// print(is_shared(x));        // prints true
/// ```
fn is_shared(variable: ?) -> bool;

/// Evaluate a text script within the current scope.
/// 
/// # Example
/// 
/// ```rhai
/// let x = 42;
/// 
/// eval("let y = x; x = 123;");
/// 
/// print(x);           // prints 123
/// print(y);           // prints 42
/// ```
fn eval(script: String) -> ?;

/// Return `true` if the string contains another string.
///
/// This function also drives the `in` operator.
///
/// # Example
///
/// ```rhai
/// let x = "hello world!";
///
/// // The 'in' operator calls 'contains' in the background
/// if "world" in x {
///     print("found!");
/// }
/// ```
fn contains(string: String, find: String) -> bool;

/// Return `true` if the string contains a character.
///
/// This function also drives the `in` operator.
///
/// # Example
///
/// ```rhai
/// let x = "hello world!";
///
/// // The 'in' operator calls 'contains' in the background
/// if 'w' in x {
///     print("found!");
/// }
/// ```
fn contains(string: String, ch: char) -> bool;

/// Return `true` if a value falls within the exclusive range.
///
/// This function also drives the `in` operator.
///
/// # Example
///
/// ```rhai
/// let r = 1..100;
///
/// // The 'in' operator calls 'contains' in the background
/// if 42 in r {
///     print("found!");
/// }
/// ```
fn contains(range: Range<int>, value: int) -> bool;

/// Return `true` if a value falls within the inclusive range.
///
/// This function also drives the `in` operator.
///
/// # Example
///
/// ```rhai
/// let r = 1..=100;
///
/// // The 'in' operator calls 'contains' in the background
/// if 42 in r {
///     print("found!");
/// }
/// ```
fn contains(range: RangeInclusive<int>, value: int) -> bool;

/// Return `true` if a key exists within the object map.
///
/// This function also drives the `in` operator.
///
/// # Example
///
/// ```rhai
/// let m = #{a:1, b:2, c:3};
///
/// // The 'in' operator calls 'contains' in the background
/// if "c" in m {
///     print("found!");
/// }
/// ```
fn contains(map: Map, string: String) -> bool;

/// Return `true` if a value is found within the BLOB.
///
/// This function also drives the `in` operator.
///
/// # Example
///
/// ```rhai
/// let b = blob();
///
/// b += 1; b += 2; b += 3; b += 4; b += 5;
///
/// // The 'in' operator calls 'contains' in the background
/// if 3 in b {
///     print("found!");
/// }
/// ```
fn contains(blob: Blob, value: int) -> bool;
