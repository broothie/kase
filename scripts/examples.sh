#! env sh

function example() {
    echo "$ kase $@"
    target/debug/kase $@
}

example snake MyVariable
echo

example screaming-snake my_variable
echo

example kebab MY_VARIABLE
echo

example path my-variable
echo

example dot my/variable
echo

example camel my.variable
echo

example pascal myVariable
echo
echo

example dot my_dir/my_path
echo

example --from path dot my_dir/my_path
echo
