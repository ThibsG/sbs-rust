Module parsing raw SBS-1 frames used by airplanes SBS units.

This module performs deserialization using Serde crate, with custom adjustment.

Messages can be 11 fields long or 22 fields long, depending on the type of message sent.

Each field is a specific type that could either be a defined type (float, unsigned) or an enum type.

Example of SBS messages:
```
SEL,,496,2286,4CA4E5,27215,2010/02/19,18:06:07.710,2010/02/19,18:06:07.710,RYR1427
ID,,496,7162,405637,27928,2010/02/19,18:06:07.115,2010/02/19,18:06:07.115,EZY691A
AIR,,496,5906,400F01,27931,2010/02/19,18:06:07.128,2010/02/19,18:06:07.128
STA,,5,179,400AE7,10103,2008/11/28,14:58:51.153,2008/11/28,14:58:51.153,RM
CLK,,496,-1,,-1,2010/02/19,18:18:19.036,2010/02/19,18:18:19.036
MSG,1,145,256,7404F2,11267,2008/11/28,23:48:18.611,2008/11/28,23:53:19.161,RJA1118,,,,,,,,,,,
```

Source: http://woodair.net/SBS/Article/Barebones42_Socket_Data.htm
