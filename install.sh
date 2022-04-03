if [ -e charify ]
then
	mv charify /usr/local/bin/
else
	echo "'charify' not found"
	exit 1
fi
