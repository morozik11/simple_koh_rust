import React from 'react';
 
class FilesList extends React.Component {
	
	constructor(props) {
		
        super(props);
        
    }

    Show(name){

        var f = this.props.functionShow.bind(this);
        f(name);

    }

	render() {
		
		let list = this.props.list_files.map((file) => {
            return <tr key={file.id}> 
				<td className="name_file firts_two_files">{file.name}</td> 
				<td className="size_file firts_two_files">{file.size}</td> 
				<td className="type_file last_files">{file.type}</td> 
                <td><button className="btn btn-default"  onClick={()=>this.Show(file.name)}>Показать</button></td></tr>
		});
		
		return <table className={this.props.classNameList}>{list}</table>;
		
	}
	
}

export default FilesList;
