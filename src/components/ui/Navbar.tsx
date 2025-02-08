import {Center, Stack, Tooltip, UnstyledButton} from '@mantine/core';
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome';
import {
    faHome,
    faGauge,
    faCalendar,
    faUser,
    faGear,
    faRightFromBracket,
    IconDefinition
} from "@fortawesome/free-solid-svg-icons";

import EyeIcon from '@/components/logos/EyeIcon';

interface NavbarLinkProps {
    icon: IconDefinition;
    label: string;
    active?: boolean;
    onClick?: () => void;
}

function NavbarLink({icon: Icon, label, active, onClick}: NavbarLinkProps) {
    return (
        <Tooltip label={label} position="right" transitionProps={{duration: 0}}>
            <UnstyledButton onClick={onClick} className="link" data-active={active || undefined}>
                <FontAwesomeIcon size="lg" stroke="1.5" icon={Icon}/>
            </UnstyledButton>
        </Tooltip>
    );
}

const mockdata = [
    {icon: faHome, label: 'Home'},
    {icon: faGauge, label: 'Dashboard'},
    {icon: faCalendar, label: 'Releases'},
    {icon: faUser, label: 'Account'},
    {icon: faGear, label: 'Settings'},
];

export default function Navbar() {
    const links = mockdata.map((link) => (
        <NavbarLink
            {...link}
            key={link.label}
            active={false}
        />
    ));

    return (
        <nav
            className="w-20 h-full p-(--mantine-spacing-md) flex flex-col navbar">
            <Center>
                <EyeIcon/>
            </Center>

            <div className="flex-1 mt-[50px]">
                <Stack justify="center" gap={0}>
                    {links}
                </Stack>
            </div>

            <Stack justify="center" gap={0}>
                <NavbarLink icon={faRightFromBracket} label="Logout"/>
            </Stack>
        </nav>
    );
}