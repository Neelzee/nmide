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
typedef struct CHtmlNode {
  CElement kind;
  struct CHtmlNode *kids;
  int kid_count;
} CHtmlNode;

typedef struct CHtmlText {
  char *text;
  int len;
} CHtmlText;

typedef union CHtmlUnion {
  CHtmlNode node;
  CHtmlText text;
} CHtmlUnion;

typedef struct CHtml {
  CHtmlUnion node;
  int isNode;
} CHtml;

#endif // !HTML_LIB
