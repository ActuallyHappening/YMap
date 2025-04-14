print "Compiling using grass ..."
do --ignore-errors { grass /home/ah/Desktop/YMap/app/styles/main.scss /home/ah/Desktop/YMap/app/public/bundled.css }
cp /home/ah/Desktop/YMap/app/public/bundled.css /home/ah/Desktop/YMap/app/dist/_/bundled.css
