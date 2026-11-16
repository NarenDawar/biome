---
"@biomejs/biome": patch
---

Added the nursery rule [`noBiomeRepeatedKindSetConstruction`](https://biomejs.dev/linter/rules/no-biome-repeated-kind-set-construction/), which reports Biome JS plugins that create kind sets with `nodeKinds()`, `tokenKinds()`, or `union()` inside a function, where the same set is created again on every call. The rule belongs to the `biomePlugin` domain.

```js
import { defineRule, nodeKinds } from "@biomejs/runtime/plugin";

export const rule = defineRule({
  run(node) {
    return nodeKinds("JS_CALL_EXPRESSION").has(node);
  },
});
```
