let 

type Date ~
record
 y: Integer,
m: Integer,
  d: Integer
  end;

type Person ~
 record 
    initials
    : Char,
    married: Boolean


    ,
    dob: Date
  end;

  proc displayperson(p: Person) ~
    begin
      put(p.initials);
      puteol();
      if p.married = true
        then put('y')
        else put('n');
      puteol();
      putint(p.dob.y);
                put(' ');
      putint(p.dob.m);
      put(' ') ! print a space

      ;
      putint(p.dob.d);
      put(' ')
    end;

var bob

: Person
in
  

  begin
    bob := { ! this is a record expression
             ! that simply initialises the variable
             ! with the ! literal
                      initials ~ 'B'
                      , married ~ false,


                      dob ~ {
                        y 
                        ~
                        1970, m ~ 12, d ~ 22 } };
    displayperson(bob) ! displays bob's information



  end
