# rshtmllog
Rust application that converts a log file to a static website. The resulting output file includes CSS styling, and JavaScript for filtering logs based on log level. Currently the parsing of the log file use log level keywords found in a typical log4j log to determine the styling of each entry.



## Filters ![image-20210328132248557](http://todo-programming.com/img/rshtmlog/img0.png)



1) When the border of a filter button is white/gray it is inactive, and each line with the corresponding level will still be visible in the log. 

![image-20210328132448600](http://todo-programming.com/img/rshtmlog/img1.png)

![image-20210328132508829](http://todo-programming.com/img/rshtmlog/img2.png)

2) When the border of a filter is green the corresponding items at that level are hidden from the view.

![image-20210328132534653](http://todo-programming.com/img/rshtmlog/img3.png)

![image-20210328132555486](http://todo-programming.com/img/rshtmlog/img4.png)

