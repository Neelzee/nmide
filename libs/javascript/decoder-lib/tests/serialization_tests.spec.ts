/**
 * Tests that the decoding work as expected.
 *
 * There is no need to test the encoding, as then we are just testing the
 * `io-ts` library. If decoding works, then encoding follows.
 */

import * as fs from "fs"
import { expect, test, suite } from "vitest";
import { DAttr } from "../lib/attr_decoder";
import { DEvent } from "../lib/event_decoder";
import { DValue } from "../lib/value_decoder";
import { DHtml } from "../lib/html_decoder";
import { DInstrAttr, DInstrHtml, DInstrString, DInstrValue } from "../lib/instr_decoder";


type Test = {
  name: string,
  input: any,
}

type TestSuite = {
  suite: string,
  tests: Test[],
};

const attrTestData: object = JSON.parse(fs.readFileSync("../../test-libs/test-data/attr-test-data.json").toString());

const attrSuites: TestSuite[] = Object.keys(attrTestData).map(k => {
  return {
    suite: k,
    // @ts-expect-error This is valid
    tests: attrTestData[k]
  }
});

attrSuites.forEach(({ suite: s, tests }) => {
  suite(s, () => {
    tests.forEach(({ name, input }) => {
      test(name, () => {
        expect(DAttr.decode(input)._tag).toBe("Right")
      })
    })
  });
});


const eventTestData: object = JSON.parse(fs.readFileSync("../../test-libs/test-data/event-test-data.json").toString());

const eventSuites: TestSuite[] = Object.keys(eventTestData).map(k => {
  return {
    suite: k,
    // @ts-expect-error This is valid
    tests: eventTestData[k]
  }
});

eventSuites.forEach(({ suite: s, tests }) => {
  suite(s, () => {
    tests.forEach(({ name, input }) => {
      test(name, () => {
        expect(DEvent.decode(input)._tag).toBe("Right")
      })
    })
  });
});


const valueTestData: object = JSON.parse(fs.readFileSync("../../test-libs/test-data/value-test-data.json").toString());

const valueSuites: TestSuite[] = Object.keys(valueTestData).map(k => {
  return {
    suite: k,
    // @ts-expect-error This is valid
    tests: valueTestData[k]
  }
});

valueSuites.forEach(({ suite: s, tests }) => {
  suite(s, () => {
    tests.forEach(({ name, input }) => {
      test(name, () => {
        expect(DValue.decode(input)._tag).toBe("Right")
      })
    })
  });
});


const htmlTestData: object = JSON.parse(fs.readFileSync("../../test-libs/test-data/html-test-data.json").toString());

const htmlSuites: TestSuite[] = Object.keys(htmlTestData).map(k => {
  return {
    suite: k,
    // @ts-expect-error This is valid
    tests: htmlTestData[k]
  }
});

htmlSuites.forEach(({ suite: s, tests }) => {
  suite(s, () => {
    tests.forEach(({ name, input }) => {
      test(name, () => {
        expect(DHtml.decode(input)._tag).toBe("Right")
      })
    })
  });
});

const instrTestData: object = JSON.parse(fs.readFileSync("../../test-libs/test-data/instr-test-data.json").toString());

const instrSuites: TestSuite[] = Object.keys(instrTestData).map(k => {
  return {
    suite: k,
    // @ts-expect-error This is valid
    tests: instrTestData[k]
  }
});

instrSuites.forEach(({ suite: s, tests }) => {
  suite(s, () => {
    tests.forEach(({ name, input }) => {
      test(name, () => {
        const value = DInstrValue.decode(input);
        const html = DInstrHtml.decode(input);
        const attr = DInstrAttr.decode(input);
        const str = DInstrString.decode(input);
        expect(
          [value._tag, html._tag, attr._tag, str._tag],
          `Should be able to decode instruction: "${input}"`
        ).toContain("Right");
      })
    })
  });
});
