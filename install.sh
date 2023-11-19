# if env is microsoft windows
if [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
    echo "windows detected"
    # if rust doesnt exist
    if ! command -v rustc &> /dev/null
    then
        echo "installing rust & cargo [Blazing fast]"
        # download rustup-init.exe with url: https://forge.rust-lang.org/infra/other-installation-methods.html#:~:text=download%20and%20run-,rustup%2Dinit.exe,-.
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.exe
        # install rustup-init.exe
        rustup-init.exe -y
        # test cargo
        cargo --version
    fi
fi

# if env is linux
if [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "linux detected"
    # if rust doesnt exist
    if ! command -v rustc &> /dev/null
    then
        echo "installing rust & cargo [Blazing fast]"
        # install rust & cargo
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        # test cargo
        cargo --version
    fi
fi

# if env is mac
if [ "$(expr substr $(uname -s) 1 5)" == "Darwin" ]; then
    echo "mac detected"
    # if rust doesnt exist
    if ! command -v rustc &> /dev/null
    then
        echo "installing rust & cargo [Blazing fast]"
        # install rust & cargo
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        # test cargo
        cargo --version
    fi
fi

cd server
# install dependencies
cargo install --path .
# navigate back
cd ..