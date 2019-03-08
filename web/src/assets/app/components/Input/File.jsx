import React from 'react';


class File extends React.Component {
	
	constructor(props) {
		
		super(props);
		this.onChange_ = this.onChange_.bind(this);
		
	}
	
	onChange_(event) {
		
		var f = this.props.functionOnchange.bind(this);
		f(event);
		
	}
	
	render() {
		
		let t = this.props.files_inputs.map((text,index) => {
			
			if(text.id == this.props.idInput){
				
				return <div id={this.props.idWrap} className={this.props.classNameWrap} key={index}>
						<label>{this.props.title}</label>
						<input multiple type="file" className={this.props.classNameInput} key={index} id={text.id} onChange={this.onChange_} /></div>;
				
			}
			
		});
		
		return t;
		
		
	}
	
};

export default File;
