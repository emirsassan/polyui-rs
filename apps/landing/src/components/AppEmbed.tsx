/* eslint-disable react-hooks/exhaustive-deps */
import clsx from 'clsx';

const AppEmbed = () => {

	return (
		<div className="w-screen">
			<div className="relative z-30 h-[255px] px-1 sm:h-[428px] md:h-[428px] lg:h-[628px] mt-8 sm:mt-16">
				<div
					className={clsx(
						'relative h-full m-auto border rounded-lg max-w-7xl transition-opacity bg-gray-850 border-gray-550 opacity-0',
						'bg-transparent border-none'
					)}
				>
					<div className="z-40 h-full sm:w-auto fade-in-app-embed landing-img">
                        <img src="https://landingimage.uwu"></img>
                    </div>
				</div>
			</div>
		</div>
	);
};

export const AppEmbedPlaceholder = () => {
	return (
		<div className="w-screen relative z-30 h-[228px] px-5 sm:h-[428px] md:h-[428px] lg:h-[628px] mt-8 sm:mt-16" />
	);
};

export default AppEmbed;