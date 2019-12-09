#!/bin/sh

$PYMUT_PATH -m Explore -d mutations.db -f mymath.py
$PYMUT_PATH -m Execute -d mutations.db -f test.py
sqlite3 -column mutations.db "select * from results;"