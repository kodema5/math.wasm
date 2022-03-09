import { assertEquals, assertExists, assertAlmostEquals } from "https://deno.land/std@0.128.0/testing/asserts.ts";

import init, * as math from "../pkg/math.js"


Deno.test("probability distribution", async(t) => {
    await init()
    window.math = math

    await t.step("normal", async () => {
        assertExists(math.normpdf)
        assertExists(math.normcdf)
        assertExists(math.norminv)

        assertAlmostEquals(math.normpdf(1,2,3), 0.125794409230998)
        assertAlmostEquals(math.normcdf(1,2,3), 0.369441340181764)
        assertAlmostEquals(math.norminv(.1,2,3), -1.844654696633802)
    })

    await t.step("student-t", async () => {
        assertExists(math.tpdf)
        assertExists(math.tcdf)

        assertAlmostEquals(math.tpdf(1,2), 0.192450089729875)
        assertAlmostEquals(math.tcdf(.1,2), 0.535267280792930)
        assertAlmostEquals(math.tcdf(2,3), 0.930337015720578)
        assertAlmostEquals(math.tcdf(-2,3), 0.069662984279422)
    })
})

