#ifndef CINTERFACE

#define CINTERFACE

#include "../chtml/chtml.h"
#include "../cmodel/cmodel.h"
#include "../cmsg/cmsg.h"

CModel init();

CModel update(CMsg msg, CModel model);

CHtml view(CModel model);

#endif // !CINTERFACE
