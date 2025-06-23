echo 'Removing module:' $1
rm -rf src/$1

echo 'Removing tests file'
rm tests/$1.rs

echo 'Finished'