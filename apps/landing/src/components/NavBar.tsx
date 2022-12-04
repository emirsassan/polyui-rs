import { Academia, Discord, Github } from '@icons-pack/react-simple-icons';
import { getWindow } from '../utils';
import { Button, Dropdown } from '@polyui/ui';
import clsx from 'clsx';
import { PropsWithChildren, useEffect, useState } from 'react';
import { Book, DotsThreeVertical } from 'phosphor-react';

function NavLink(props: PropsWithChildren<{ link?: string }>) {
	return (
		<a
			href={props.link ?? '#'}
			target={props.link?.startsWith('http') ? '_blank' : undefined}
			className="p-4 text-gray-300 no-underline transition cursor-pointer hover:text-gray-50"
			rel="norefferer"
		>
			{props.children}
		</a>
	);
}

function link(path: string) {
	return {
		selected: getWindow()?.location.href.includes(path),
		onClick: () => getWindow()?.location // router$.nav
	};
}

function redirect(href: string) {
	return () => (window.location.href = href);
}

export default function NavBar() {
	const [isAtTop, setIsAtTop] = useState(true);
	const window = getWindow();

	function onScroll() {
		if ((getWindow()?.pageYOffset || 0) > 20) setIsAtTop(true);
		else if (isAtTop) setIsAtTop(false);
	}

	useEffect(() => {
		if (!window) return;

		setTimeout(onScroll, 0);
		getWindow()?.addEventListener('scroll', onScroll);
		return () => getWindow()?.removeEventListener('scroll', onScroll);
	}, []);

	return (
		<div
			className={clsx(
				'fixed transition px-2 z-[55] w-full h-16 border-b ',
				isAtTop
					? 'bg-transparent border-transparent'
					: 'border-gray-550 bg-gray-700 bg-opacity-80 backdrop-blur'
			)}
		>
			<div className="relative flex max-w-[100rem] mx-auto items-center h-full m-auto p-5">
				<a href="/" className="absolute flex flex-row items-center">
					<img className="z-30 w-8 h-8 mr-3" />
					<h3 className="text-xl font-bold text-white">Polyfrost</h3>
				</a>

				<div className="flex-1 lg:hidden" />
				<Dropdown.Root
					button={
						<Button className="ml-[140px] hover:!bg-transparent" size="icon">
							<DotsThreeVertical weight="bold" className="w-6 h-6 " />
						</Button>
					}
					className="block h-6 text-white w-44 top-2 right-4 lg:hidden"
					itemsClassName="!rounded-2xl shadow-2xl shadow-black p-2 !bg-gray-850 mt-2 !border-gray-500 text-[15px]"
				>
					<Dropdown.Section>
						<Dropdown.Item
							icon={Github}
							onClick={redirect('https://github.com/polyfrost')}
						>
							GitHub
						</Dropdown.Item>
						<Dropdown.Item icon={Discord} onClick={redirect('https://polyfrost.cc/discord')}>
							Join Discord
						</Dropdown.Item>
					</Dropdown.Section>
					<Dropdown.Section>
						<Dropdown.Item icon={Book} {...redirect('https://docs.polyfrost.cc')}>
							Docs
						</Dropdown.Item>
					</Dropdown.Section>
				</Dropdown.Root>

				<div className="absolute flex-row hidden space-x-5 right-3 lg:flex">
					<a href="https://polyfrost.cc/discord" target="_blank" rel="noreferrer">
						<Discord className="text-white" />
					</a>
					<a href="https://github.com/polyfrost" target="_blank" rel="noreferrer">
						<Github className="text-white" />
					</a>
					<a href="https://docs.polyfrost.cc" target="_blank" rel="norefferer">
						<Book className="text-white" />
					</a>
				</div>
			</div>
		</div>
	);
}
