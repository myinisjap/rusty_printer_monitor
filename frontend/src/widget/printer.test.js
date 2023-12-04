import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import  PrinterWidget from './printer';

describe('PrinterWidget', () => {
    const mockProps = {
        printer_name: 'Test Printer',
        ip_address: '192.168.0.100',
        files_available: ['file1', 'file2', 'file3'],
        progress: 50,
    };

    beforeEach(() => {
        render(<PrinterWidget {...mockProps} />);
    });

    test('renders printer name in the DOM', () => {
        expect(screen.getByTitle(mockProps.printer_name)).toBeInTheDocument();
    });

    test('renders IP Address in the DOM', () => {
        expect(screen.getByText(`${mockProps.ip_address}`)).toBeInTheDocument();
    });

    test('renders all files in the files list', () => {
        mockProps.files_available.forEach(file => {
            expect(screen.getByText(file)).toBeInTheDocument();
        });
    });

    // test('calls deleteWidget function when delete button is clicked', () => {
    //     fireEvent.click(screen.getByText("X"));
    //     expect(deleteWidget).toHaveBeenCalled();
    //     expect(deleteWidget).toHaveBeenCalledWith(mockProps.printer_name);
    // });

    // Add more tests as necessary...
});
