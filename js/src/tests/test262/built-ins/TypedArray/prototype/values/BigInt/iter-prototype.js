// Copyright (C) 2016 the V8 project authors. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
es6id: 22.2.3.30
esid: sec-%typedarray%.prototype.values
description: >
  The prototype of the returned iterator is ArrayIteratorPrototype
info: |
  22.2.3.30 %TypedArray%.prototype.values ( )

  ...
  3. Return CreateArrayIterator(O, "value").
includes: [testBigIntTypedArray.js]
features: [BigInt, Symbol.iterator, TypedArray]
---*/

var ArrayIteratorProto = Object.getPrototypeOf([][Symbol.iterator]());

testWithBigIntTypedArrayConstructors(function(TA) {
  var sample = new TA([0n, 42n, 64n]);
  var iter = sample.values();

  assert.sameValue(Object.getPrototypeOf(iter), ArrayIteratorProto);
});

reportCompare(0, 0);
