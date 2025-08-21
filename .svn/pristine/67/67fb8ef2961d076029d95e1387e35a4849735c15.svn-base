# Foreign Language Library Test Suite

## 1. Type Construction Tests

### Html Type Tests
- **Basic element construction**: Create simple elements like `div`, `span`, `p`
- **Nested elements**: Create elements with children
- **Text content**: Elements with text content, empty text, and null text
- **Mixed content**: Elements with both text and child elements
- **Self-closing elements**: `img`, `br`, `hr`, `input` elements
- **Complex nesting**: Deeply nested structures

### Attr Type Tests
- **ID attributes**: Basic id attribute creation
- **Class attributes**: Single and multiple class names
- **Event attributes**: Click events and other event types
- **Custom attributes**: Data attributes, aria attributes, any key-value pairs
- **Multiple attributes**: Elements with combinations of all attribute types

### Value Type Tests
- **Null values**: Default null value handling
- **Integers**: Positive, negative, zero
- **Floats**: Positive, negative, zero, small decimals, special float values
- **Booleans**: True and false values
- **Strings**: Empty strings, Unicode characters, special characters, very long
  strings
- **Lists**: Empty lists, homogeneous lists, heterogeneous lists, nested lists
- **Objects**: Empty objects, simple key-value pairs, nested objects, objects
  with all Value types
- **Html in Value**: Html elements stored as Value type

### Event Type Tests
- **Event with data**: Events carrying additional information
- **Custom events**: User-defined event types

## 2. Serialization/Deserialization Tests

### JavaScript Target Languages (Gleam, PureScript, etc.)
- **Html to JavaScript**: Serialize Html types to JavaScript objects
- **Html from JavaScript**: Deserialize JavaScript objects back to Html types
- **Value to JavaScript**: Serialize all Value variants to JavaScript
- **Value from JavaScript**: Deserialize JavaScript back to Value types
- **Attr to JavaScript**: Serialize attributes to JavaScript representation
- **Attr from JavaScript**: Deserialize JavaScript back to Attr types
- **Event to JavaScript**: Serialize events to JavaScript
- **Event from JavaScript**: Deserialize JavaScript back to Event types
- **CoreModification to JavaScript**: Serialize Instructions to JavaScript
- **CoreModification from JavaScript**: Deserialize JavaScript back to
  Instructions

### Rust Target Languages
- **Html to Rust**: Serialize Html types to Rust-compatible format
- **Html from Rust**: Deserialize from Rust format back to Html types
- **Value to Rust**: Serialize all Value variants to Rust format
- **Value from Rust**: Deserialize Rust format back to Value types
- **Attr to Rust**: Serialize attributes to Rust representation
- **Attr from Rust**: Deserialize Rust format back to Attr types
- **Event to Rust**: Serialize events to Rust format
- **Event from Rust**: Deserialize Rust format back to Event types
- **CoreModification to Rust**: Serialize Instructions to Rust format
- **CoreModification from Rust**: Deserialize Rust format back to Instructions

## 3. Type Manipulation Tests

### Html Manipulation
- **Add children**: Add single and multiple child elements
- **Remove children**: Remove specific children by index or reference
- **Modify text**: Change text content of elements
- **Add attributes**: Add new attributes to existing elements
- **Remove attributes**: Remove specific attributes
- **Modify attributes**: Change existing attribute values
- **Get Element By _**: Can navigate the HTML tree

### Value Manipulation
- **List operations**: Add, remove, modify list elements
- **Object operations**: Add, remove, modify object properties
- **Type conversions**: Convert between compatible Value types
- **Deep access**: Access nested values in complex structures
- **Deep modification**: Modify nested values in complex structures

### Attr Manipulation
- **Class manipulation**: Fold attributes
- **Custom attribute manipulation**: Modify custom attribute values

## 4. CoreModification/Instruction Optimization Tests

### Basic Optimization
- **NoOp removal**: Remove all "NoOp" instructions from chains
- **Add/Remove cancellation**: Detect when adding then removing same field
  results in NoOp
- **Nested Add/Removal cancellation**: Detect when adding and removing the same
  field in a nested tree results in NoOp
- **Consecutive operations**: Optimize multiple operations on same field

### Advanced Optimization
- **Nested Then chains**: Flatten nested "then" operations
- **Idempotent operations**: Detect operations that don't change state

### Instruction Construction
- **Single operations**: Create individual add/remove/noOp instructions
- **Chained operations**: Create complex "then" chains
- **Parameterized instructions**: Test with Value, Html, Attr, and String
  parameters
- **Empty instruction chains**: Handle edge cases with empty operations

## 5. Cross-Language Compatibility Tests

### Round-trip Tests
- **Html round-trip**: Foreign language → JavaScript/Rust → Foreign language
- **Value round-trip**: Test all Value variants through complete cycle
- **Attr round-trip**: Test all attribute types through complete cycle
- **Instruction round-trip**: Test all instruction types through complete cycle

### Edge Case Handling
- **Null handling**: Consistent null/undefined behavior across languages
- **Empty collections**: Empty lists, objects, and structures
- **Unicode handling**: Proper encoding/decoding of unicode strings
- **Large data**: Performance with large Html structures and Value objects
- **Deeply nested data**: Handle deeply nested structures without stack overflow

## 6. Error Handling Tests

### Invalid Input Tests
- **Malformed JavaScript**: Handle invalid JavaScript representations
- **Malformed Rust**: Handle invalid Rust representations
- **Type mismatches**: Handle incorrect type assignments
- **Missing required fields**: Handle incomplete data structures

### Boundary Condition Tests
- **Maximum values**: Test with maximum integer values, very long strings
- **Minimum values**: Test with minimum integer values, empty strings
- **Resource limits**: Test behavior under memory constraints
- **Performance limits**: Test with very large data structures
