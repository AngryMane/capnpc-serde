@0xef41c006a99a86cb;

using Phone = import "phone.capnp";

annotation note (*) :Text;
annotation baz(*) :Int32;

$note("annotation to file");


interface Sample {
    initialize @0 (debug: Bool) -> (result: Bool);

    interface Subscriber {
        pushMessage @0 () -> (result: Bool);
    }

    subscribe @1 (subscriber: Subscriber) -> (result: Bool);
}


struct Daty $baz(2) {
  enum EnumTest {
    foo @0;
    bar @1;
    baz @2;
    qux @3;
    # ...
  }
  year @0 :EnumTest = foo;
  month @1 :UInt8;
  day @2 :UInt8;
  target @3 :List(Bool) = [ true, false, false, true ];
}  
