from="\/index"
to=".\/index"

cd www &&
rm dist/* &&
trunk build --release &&
cd .. &&
rm docs/*
cp www/dist/* docs/ &&
sed -i -e "s/$from/$to/g" ./docs/index.html
rm docs/*.html-e
git add docs/*