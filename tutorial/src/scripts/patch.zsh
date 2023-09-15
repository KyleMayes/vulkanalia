if [[ $# -ne 3 ]]; then
    echo "Expected original file, patched file, and prefix number to start applying patch to."
    exit
fi

ORIGINAL=$1
PATCHED=$2
START=$3

diff -Naur $ORIGINAL $PATCHED > patch.patch

for FILE in *.rs; do
    PREFIX=$((10#${FILE%%_*}))
    if (( $PREFIX >= $START )) && [[ $FILE != $PATCHED ]]; then
        patch -f $FILE < patch.patch

        if [[ $? -ne 0 ]]; then
            echo "Failed to apply ($FILE)."
            mv ${FILE}.orig $FILE
        fi

        rm -f ${FILE}.orig
        rm -f ${FILE}.rej
    fi
done

rm -f patch.patch
