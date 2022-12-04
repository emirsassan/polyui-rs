import { Discord, Github, Twitter } from '@icons-pack/react-simple-icons';
import { PropsWithChildren } from 'react';

function FooterLink(props: PropsWithChildren<{ link: string; blank?: boolean }>) {
	return (
		<a
			href={props.link}
			target={props.blank ? '_blank' : ''}
			className="text-gray-300 hover:text-white"
			rel="norefferer"
		>
			{props.children}
		</a>
	);
}

export function Footer() {
	return (
		<footer id="footer" className="z-50 w-screen pt-3 border-t border-gray-550 bg-gray-850">
			<div className="max-w-[100rem] mx-auto grid grid-cols-2 gap-6 p-8 pt-10 pb-20 m-auto text-white min-h-64 sm:grid-cols-2 lg:grid-cols-6">
				<div className="col-span-2">
					<h3 className="mb-1 text-xl font-bold">Polyfrost</h3>
					<p className="text-sm text-gray-350">&copy; Copyright 2022 Polyfrost</p>
					<div className="flex flex-row mt-6 mb-10 space-x-3">
						<FooterLink link="https://twitter.com/polyfrost">
							<Twitter />
						</FooterLink>
						<FooterLink link="https://polyfrost.cc/discord">
							<Discord />
						</FooterLink>
						<FooterLink link="https://github.com/polyfrost">
							<Github />
						</FooterLink>
					</div>
				</div>

				<div className="flex flex-col col-span-1 space-y-2">
					<h3 className="mb-1 text-xs font-bold uppercase ">About</h3>
				</div>

				<div className="flex flex-col col-span-1 space-y-2">
					<h3 className="mb-1 text-xs font-bold uppercase ">Developers</h3>
					<FooterLink blank link="https://docs.polyfrost.cc">
						Documentation
					</FooterLink>
					<FooterLink blank link="https://github.com/polyfrost/oneconfig/blob/main/CONTRIBUTING.md">
						Contribute
					</FooterLink>
				</div>
			</div>
		</footer>
	);
}
