#ifndef HTML_LIB

#define HTML_LIB

/**
 * Enumeration of HTML-Elements
 **/
typedef enum CElement {
  Div,
  P,
  Span,
  Section,
  Input,
  Button,
  Text,
  Script,
  Select,
  Aside,
  Nav,
  A,
} CElement;

/**
 * Struct representation of an HTML node
 **/
typedef struct CHtml {
  CElement kind;
  int attrs;
  struct CHtml *kids;
  int kid_count;
} CHtml;

#endif // !HTML_LIB
