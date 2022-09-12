import { useEffect, useState } from 'react';

import { Platform } from '@stump/client';
import { os } from '@tauri-apps/api';

import StumpInterface from '@stump/interface';

export default function App() {
	function getPlatform(platform: string): Platform {
		console.log('getPlatform', platform);
		switch (platform) {
			case 'darwin':
				return 'macOS';
			case 'win32':
				return 'windows';
			case 'linux':
				return 'linux';
			default:
				return 'browser';
		}
	}

	const [platform, setPlatform] = useState<Platform>('unknown');

	useEffect(() => {
		os.platform().then((platform) => setPlatform(getPlatform(platform)));
	}, []);

	return <StumpInterface platform={platform} />;
}
