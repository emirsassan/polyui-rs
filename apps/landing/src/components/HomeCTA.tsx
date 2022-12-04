import { Github } from '@icons-pack/react-simple-icons';
import { Button } from '@polyui/ui';

export function HomeCTA() {

	return (
		<>
			<div className="z-30 flex flex-row items-center h-10 space-x-4 animation-delay-2 fade-in">
					<>
						<Button
							href="https://github.com/polyfrost/oneconfig"
							target="_blank"
							className="z-30 cursor-pointer"
							variant="accent"
						>
							<Github className="inline w-5 h-5 -mt-[4px] -ml-1 mr-2" fill="white" />
							Star on GitHub
						</Button>
					</>	
			</div>
		</>
	);
}

export default HomeCTA;