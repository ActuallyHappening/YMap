
def compile () {
    print "Compiling using grass ..."
    do --ignore-errors { grass /home/ah/Desktop/YMap/app/styles/main.scss /home/ah/Desktop/YMap/app/dist/_/bundled.css }
}
compile;
watch /home/ah/Desktop/YMap/app/ --glob **/*.scss {|| compile}
