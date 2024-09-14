@0xef41c006a99a86cb;

using Phone = import "phone.capnp";

annotation note (*) :Text;
annotation baz(*) :Int32;

$note("annotation to file");

interface Example extends(Sample) {
    finalize @0 (debug: Bool) -> (result: GenericsIF(Daty));
}

interface Sample {
    initialize @0 (debug: Phone.Date(Daty) ) -> (result: GenericsIF(Daty));

    interface Subscriber {
        pushMessage @0 () -> (result: Bool);
    }

    subscribe @1 (subscriber: Subscriber) -> (result: Bool);
}

interface GenericsIF(T) {
    testMessage @0 () -> (result: T);
}

struct Daty(T) $baz(2) {
  enum EnumTest {
    foo @0;
    bar @1;
    baz @2;
    qux @3;
    # ...
  }
  year @0 :EnumTest = foo;
  month @1 :T;
  day @2 :UInt8;
  target @3 :List(Bool) = [ true, false, false, true ];
}  
