 (* A sample of Oberon code *)
MODULE Samples;

    (* Multiply three integers together *)
    PROCEDURE Multiply*;
    VAR x, y, z: INTEGER;
    BEGIN OpenInput; ReadInt(x); ReadInt(y); z := 0;
    WHILE x > 0 DO
        IF x MOD 2 = 1 THEN z := z + y END ;
        y := 2*y; x := x DIV 2
    END ;
    WriteInt(x, 4); WriteInt(y, 4); WriteInt(z, 6); WriteLn
    END Multiply;
    
END Samples;