program = "Dear" {string} "," optional_{statement} "Regards" "," {expr} "."
statement = "Note that" {string} "is equal to" {expr} "."
expr = integer literal (i64)
string = &str
