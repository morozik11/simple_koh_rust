import React from 'react';
import ReactDOM from 'react-dom';
import { createStore } from 'redux'
import { Provider } from 'react-redux'


import Header from './components/Header.jsx';

import WebApp from './reducers/index.js'

import File_ from './containers/file.js';
import FilesList_ from './containers/list.js';

import {setFile, show} from './function.js';

class App extends React.Component {
	
	constructor() {
		super();
	}

	render() {
		return <main className="clearfix">
			<Header classNameHeader="header text-center" classNameLogo="" title="GRa.." />
			<div id="add_wrap">
				<File_ idWrap={'first_wrap'} classNameWrap={'wrap_files'} title={'Добавить'} classNameInput={'inputs_file'} idInput={'csv_file'} functionOnchange={setFile} />
				<FilesList_ functionShow={show} classNameList={'files_list'} />
			</div>
		</main>;
	}
	
}
 
let store = createStore(WebApp,{
	files_inputs:[
		{
			id: 'csv_file',
			files: ''
		},
	],
	list_files:[]
});

ReactDOM.render(
  <Provider store={store}>
    <App />
  </Provider>,
  document.getElementById('app')
);

export default store;
