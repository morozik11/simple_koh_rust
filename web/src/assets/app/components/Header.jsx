var React = require('react');


class Header extends React.Component {
	
	constructor(props){
        super(props);
    }
	
	render() {
        return(
            <div className={this.props.classNameHeader}>         
                <a className={this.props.classNameLogo} href="file:///home/stas/vue/index.html">{this.props.title}</a>
            </div>);
    }
	
};

module.exports = Header;
