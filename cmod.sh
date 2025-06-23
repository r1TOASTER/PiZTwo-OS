echo 'Making folder:' $1
mkdir -p src/$1

echo 'Creating module file'
touch src/$1/mod.rs

echo 'Creating tests file'
touch tests/$1.rs

echo 'Finished'