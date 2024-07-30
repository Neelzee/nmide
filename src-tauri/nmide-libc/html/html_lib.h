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

typedef struct CHtmlText {
  char *text;
  int len;
} CHtmlText;

typedef union CHtmlUnion {
  CElement kind;
  CHtmlText text;
} CHtmlUnion;

typedef struct CHtml {
  CHtmlUnion node;
  int isNode;
  struct CHtml *kids;
  int kid_count;
} CHtml;

/**
 * Empty div
 */
CHtml div();

/**
 * Empty p
 */
CHtml p();

/**
 * Empty span
 */
CHtml span();

/**
 * Empty section
 */
CHtml section();

/**
 * Empty input
 */
CHtml input();

/**
 * Empty button
 */
CHtml button();

/**
 * Empty script
 */
CHtml script();

/**
 * Empty select
 */
CHtml select();

/**
 * Empty aside
 */
CHtml aside();

/**
 * Empty nav
 */
CHtml nav();

/**
 * Empty a
 */
CHtml a();

/**
 * Empty text
 */
CHtml text();

/**
 * <div>
 *  <p>Hello, World!</p>
 * </div>
 */
CHtml simple_test();

#endif // !HTML_LIB
