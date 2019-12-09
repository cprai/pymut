#!/bin/sh

rm mutations.db
$PYMUT_PATH -m Explore -d mutations.db -f mymath.py
$PYMUT_PATH -m Execute -d mutations.db -f good_tests.py
$PYMUT_PATH -m Execute -d mutations.db -f bad_tests.py
sqlite3 -column mutations.db "select * from results;"