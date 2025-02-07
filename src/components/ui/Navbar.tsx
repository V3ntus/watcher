import {Center, Stack, Tooltip, UnstyledButton} from '@mantine/core';
import classes from './Navbar.module.css';
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
            <UnstyledButton onClick={onClick} className={classes.link} data-active={active || undefined}>
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
        <nav className={classes.navbar}>
            <Center>
                <EyeIcon/>
            </Center>

            <div className={classes.navbarMain}>
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