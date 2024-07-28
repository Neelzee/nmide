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
  None,
} CElement;

/**
 * Struct representation of an HTML node
 **/
typedef struct CHtml {
  CElement kind;
  struct CHtml *kids;
  int kid_count;
} CHtml;

CHtml create_chtml(CElement kind, CHtml *kids, int kid_count);

#endif // !HTML_LIB
