import clsx from 'clsx';
import { Helmet } from 'react-helmet';

import AppEmbed, { AppEmbedPlaceholder } from '../components/AppEmbed';
import HomeCTA from '../components/HomeCTA';

interface SectionProps {
	orientation: 'left' | 'right';
	heading?: string;
	description?: string | React.ReactNode;
	children?: React.ReactNode;
	className?: string;
}

function Section(props: SectionProps = { orientation: 'left' }) {
	const info = (
		<div className="px-4 py-10 sm:px-10">
			{props.heading && <h1 className="text-2xl font-black sm:text-4xl">{props.heading}</h1>}
			{props.description && (
				<p className="mt-5 text-md sm:text-xl text-gray-450">{props.description}</p>
			)}
		</div>
	);
	return (
		<div className={clsx('grid grid-cols-1 my-10 lg:grid-cols-2 lg:my-44', props.className)}>
			{props.orientation === 'right' ? (
				<>
					{info}
					{props.children}
				</>
			) : (
				<>
					{props.children}
					{info}
				</>
			)}
		</div>
	);
}

function Page() {
	return (
		<div className="flex flex-col items-center w-full px-4">
			<Helmet>
				<title>Polyfrost</title>
				<meta
					name="keywords"
					content="polyfrost,poly,oneconfig,oneconfig mod,minecraft mod,essential mod,config mod,minecraft config mod,minecraft mod,minecraft forge,minecraft fabric"
				/>
				<meta name="author" content="Spacedrive Technology Inc." />
			</Helmet>
			<div className="mt-22 lg:mt-28" id="content" aria-hidden="true" />
			<div className="mt-24 lg:mt-5" />

			<h1 className="z-30 px-2 mb-3 text-4xl font-black leading-tight text-center text-white fade-in-heading md:text-6xl">
				Polyfrost meow moew Oneconfgi yes eys
			</h1>
			<p className="z-30 max-w-4xl mt-1 mb-8 text-center animation-delay-1 fade-in-heading text-md lg:text-lg leading-2 lg:leading-8 text-gray-450">
				meow meow meow
				<br />
				<span className="hidden sm:block">
					meow mew
				</span>
			</p>
			<HomeCTA />
			<AppEmbed />
			<Section
				orientation="right"
				heading="Never leave a file behind."
				className="z-30 mt-0 sm:mt-8"
				description={
					<>
						meow meow
						<br />
						<br />
						<a
							className="transition text-primary-600 hover:text-primary-500"
							href="https://meow"
						>
							Find out more →
						</a>
					</>
				}
			/>
		</div>
	);
}

export { Page };