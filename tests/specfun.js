import { assertEquals, assertExists, assertAlmostEquals } from "https://deno.land/std@0.128.0/testing/asserts.ts";

import init, * as math from "../pkg/math.js"


Deno.test("specfun", async(t) => {
    await init()
    window.math = math

    await t.step("beta", async () => {
        assertExists(math.beta)
        assertExists(math.betainc)
        assertExists(math.betaincinv)

        assertAlmostEquals(math.beta(1,2), 0.5)
        assertAlmostEquals(math.betainc(.1,.2,.3), 0.412134004363243)
        assertAlmostEquals(math.betaincinv(.1,2,3), 0.142559316710031)
    })

    await t.step("erf", async () => {
        assertExists(math.erf)
        assertExists(math.erfc)
        assertExists(math.erfinv)
        assertExists(math.erfcinv)

        assertAlmostEquals(math.erf(1), 0.842700792949715)
        assertAlmostEquals(math.erfc(1), 0.157299207050285)
        assertAlmostEquals(math.erfinv(.1), 0.088855990494258)
        assertAlmostEquals(math.erfcinv(.1), 1.163087153676674)
    })

    await t.step("gamma", async () => {
        assertExists(math.gamma)
        assertExists(math.gammaln)
        assertExists(math.gammainc)
        assertExists(math.gammaincinv)

        assertAlmostEquals(math.gamma(.1), 9.513507698668731)
        assertAlmostEquals(math.gammaln(.1), 2.252712651734206)
        assertAlmostEquals(math.gammainc(.1, .2), 0.676043203815343)
        assertAlmostEquals(math.gammaincinv(.1, 2), 0.531811608389612)
    })
})

