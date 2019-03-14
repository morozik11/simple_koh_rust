import store from './app.jsx';
import { editFile, getFile, emptyFile } from './actions/files.js';
import { addElem, deleteElem } from './actions/list.js';


function setFile(event){

    store.dispatch(editFile(event.target.id,event.target.files)); 

    var listFiles = store.getState().list_files;
    var eFiles = event.target.files;

    for(var i=0; i<eFiles.length; i++){

        var result = true;

        for(var j=0; j<listFiles.length; j++){
            if(listFiles[j].name == eFiles[i].name){
                result = false;
            }
        }

        if(eFiles[i].type == 'text/csv' && result == true){
            store.dispatch(addElem(eFiles[i]));
        }

    }

}

function getRandomColor() {
    var letters = '0123456789ABCDEF'.split('');
    var color = '#';
            
    for (var i = 0; i < 6; i++ ) {
        color += letters[Math.round(Math.random() * 15)];
    }
            
    return color;
}

function showCanvas(canvas,type,labels,label,data,backgroundColor,borderColor){
    
    var ctx = document.getElementById(canvas).getContext('2d');
    ctx.height = 80;
    var myChart = new Chart(ctx, {
        type: type,
        data: {
            labels: labels,
            datasets: [{
                label: label,
                data: data,
                backgroundColor: backgroundColor,
                borderColor: [],
                borderWidth: 1
            }]
        },
        options: {
            maintainAspectRatio: true
        }
    });
}

function show(name){
    
    var listFiles = store.getState().list_files;
    var file = false;

    for(var i=0; i<listFiles.length; i++){
        if(listFiles[i].name == name){
            file = listFiles[i];
            break;
        }
    }

    if(file != false){
        
        var data = new FormData();
        data.append('file', file);
        data.append('time',new Date());
        
        $.ajax({
            url: '/upload/',
            type: 'POST',
            data: data,
            cache: false,
            dataType: 'json',
            processData: false, 
            contentType: false, 
            success: function(data){
                
                data.sort(function(a, b) { 
                    return a.class - b.class;
                });
                
                var color = getRandomColor();
                var counterData = 0;
                var BgColor = [];
                var BorColor = [];
                var data_ = [];
                var label = "";
                var labels = [];
                
                var classes = [];
                
                $(data).each(function(i,elem){
                    
                    if(classes.indexOf(elem.class) == -1){
                        
                        counterData = counterData + 1;
                        classes.push(elem.class);
                        color = getRandomColor()+"3d";
                        BgColor.push(color);
                        BorColor.push("#ffff");
                        data_.push(1);
                        labels.push(elem.class+" "+"["+elem.vector.join(',')+"]");
                        
                    } else {
                        
                        BgColor.push(color);
                        BorColor.push("rgba(255,99,132,1)");
                        data_.push(1);
                        labels.push(elem.class+" "+"["+elem.vector.join(',')+"]");
                        
                    }
                    
                });
                
                showCanvas('one','doughnut',labels,label,data_,BgColor,BorColor);
                
            },
            error: function(jqXHR, textStatus, errorThrown){
                
                console.log(textStatus);
                console.log(jqXHR);
                
            }
        });
        
    }

}

export { setFile, show };
