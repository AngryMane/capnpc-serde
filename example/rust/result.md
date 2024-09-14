classDiagram









class `pushMessage$Results` 
<<struct>> `pushMessage$Results`


`pushMessage$Results` : Bool result








class `Daty` ~T~
<<struct>> `Daty`


`Daty` : EnumTest year
`Daty` :  month
`Daty` : Uint8 day
`Daty` : List~Bool~ target
`Daty` --> EnumTest : Refernce
                class EnumTest 
<<enum>> EnumTest

EnumTest : foo
EnumTest : bar
EnumTest : baz
EnumTest : qux







class `initialize$Params` 
<<struct>> `initialize$Params`


`initialize$Params` : Date~Daty~ debug
`initialize$Params` --> Date~YEARTYPE~ : Refernce
                `initialize$Params` --> Daty~T~ : Refernce
                        







class `finalize$Params` 
<<struct>> `finalize$Params`


`finalize$Params` : Bool debug




class `GenericsIF` ~T~
<<interface>> `GenericsIF`




GenericsIF : testMessage() ( result)








class `testMessage$Params` 
<<struct>> `testMessage$Params`


`testMessage$Params` : Person a
`testMessage$Params` --> Person : Refernce
                class EnumTest 
<<enum>> EnumTest

EnumTest : foo
EnumTest : bar
EnumTest : baz
EnumTest : qux







class `subscribe$Params` 
<<struct>> `subscribe$Params`


`subscribe$Params` : Subscriber subscriber
`subscribe$Params` --> Subscriber : Refernce
                



class `Example` 
<<interface>> `Example`




Example : finalize(Bool debug) (GenericsIF~Daty~ result)
`Example` --> GenericsIF~T~ : Reference
                `Example` --> Daty~T~ : Refernce
                        







class `Dymmy` 
<<struct>> `Dymmy`


`Dymmy` : Uint16 year








class `subscribe$Results` 
<<struct>> `subscribe$Results`


`subscribe$Results` : Bool result








class `testMessage$Results` 
<<struct>> `testMessage$Results`


`testMessage$Results` : Int8 result








class `DymmyA` ~YEARTYPE~
<<struct>> `DymmyA`


`DymmyA` : Date~Dymmy~ year
`DymmyA` :  test
`DymmyA` --> Date~YEARTYPE~ : Refernce
                `DymmyA` --> Dymmy : Refernce
                        







class `finalize$Results` 
<<struct>> `finalize$Results`


`finalize$Results` : GenericsIF~Daty~ result
`finalize$Results` --> GenericsIF~T~ : Refernce
                `finalize$Results` --> Daty~T~ : Refernce
                        







class `PhoneNumber` 
<<struct>> `PhoneNumber`


`PhoneNumber` : Text number
`PhoneNumber` : Type type
`PhoneNumber` --> Type : Refernce
                class Type 
<<enum>> Type

Type : mobile
Type : home
Type : work



class `PHoneIF` 
<<interface>> `PHoneIF`




PHoneIF : testMessage(Person a) (Int8 result)
`PHoneIF` --> Person : Reference
                



class `Sample` 
<<interface>> `Sample`




Sample : initialize(Date~Daty~ debug) (GenericsIF~Daty~ result)
Sample : subscribe(Subscriber subscriber) (Bool result)
`Sample` --> Date~YEARTYPE~ : Reference
                `Sample` --> Daty~T~ : Refernce
                        `Sample` --> GenericsIF~T~ : Reference
                `Sample` --> Daty~T~ : Refernce
                        `Sample` --> Subscriber : Reference
                
    



class `Subscriber` 
<<interface>> `Subscriber`




Subscriber : pushMessage() (Bool result)








class `testMessage$Params` ~T~
<<struct>> `testMessage$Params`










class `pushMessage$Params` 
<<struct>> `pushMessage$Params`


class Type 
<<enum>> Type

Type : mobile
Type : home
Type : work







class `DymmyB` 
<<struct>> `DymmyB`


`DymmyB` : Int8 aa








class `Person` 
<<struct>> `Person`


`Person` : Text name
`Person` : Text email
`Person` : List~PhoneNumber~ phones
`Person` : Date~DymmyB~ birthdate
`Person` --> PhoneNumber : Refernce
                `Person` --> Date~YEARTYPE~ : Refernce
                `Person` --> DymmyB : Refernce
                        







class `PhoneNumber` 
<<struct>> `PhoneNumber`


`PhoneNumber` : Text number
`PhoneNumber` : Type type
`PhoneNumber` --> Type : Refernce
                class Type 
<<enum>> Type

Type : mobile
Type : home
Type : work







class `Date` ~YEARTYPE~
<<struct>> `Date`


`Date` :  year
`Date` : Uint8 month
`Date` : Uint8 day
`Date` : DymmyA test
`Date` --> DymmyA~YEARTYPE~ : Refernce
                







class `DymmyA` ~YEARTYPE~
<<struct>> `DymmyA`


`DymmyA` : Date~Dymmy~ year
`DymmyA` :  test
`DymmyA` --> Date~YEARTYPE~ : Refernce
                `DymmyA` --> Dymmy : Refernce
                        



class `Subscriber` 
<<interface>> `Subscriber`




Subscriber : pushMessage() (Bool result)








class `initialize$Results` 
<<struct>> `initialize$Results`


`initialize$Results` : GenericsIF~Daty~ result
`initialize$Results` --> GenericsIF~T~ : Refernce
                `initialize$Results` --> Daty~T~ : Refernce
                        







class `testMessage$Results` ~T~
<<struct>> `testMessage$Results`


`testMessage$Results` :  result

