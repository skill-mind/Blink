import 'react-native-gesture-handler/jestSetup';

// Mock react-native modules that don't work well in Jest
jest.mock('react-native-reanimated', () => {
  const Reanimated = require('react-native-reanimated/mock');
  Reanimated.default.call = () => {};
  return Reanimated;
});

// Mock react-native-keychain
jest.mock('react-native-keychain', () => ({
  setInternetCredentials: jest.fn(() => Promise.resolve()),
  getInternetCredentials: jest.fn(() => Promise.resolve({ username: 'test', password: 'test' })),
  resetInternetCredentials: jest.fn(() => Promise.resolve()),
  canImplyAuthentication: jest.fn(() => Promise.resolve(true)),
  getSupportedBiometryType: jest.fn(() => Promise.resolve('FaceID')),
}));

// Mock react-native-nfc-manager
jest.mock('react-native-nfc-manager', () => ({
  start: jest.fn(() => Promise.resolve()),
  isSupported: jest.fn(() => Promise.resolve(true)),
  isEnabled: jest.fn(() => Promise.resolve(true)),
  requestTechnology: jest.fn(() => Promise.resolve()),
  cancelTechnologyRequest: jest.fn(() => Promise.resolve()),
  getTag: jest.fn(() => Promise.resolve({})),
}));

// Mock react-native-biometrics
jest.mock('react-native-biometrics', () => ({
  isSensorAvailable: jest.fn(() => Promise.resolve({ available: true, biometryType: 'FaceID' })),
  createKeys: jest.fn(() => Promise.resolve({ publicKey: 'test-key' })),
  biometricKeysExist: jest.fn(() => Promise.resolve({ keysExist: true })),
  createSignature: jest.fn(() => Promise.resolve({ success: true, signature: 'test-signature' })),
}));

// Mock AsyncStorage
jest.mock('@react-native-async-storage/async-storage', () =>
  require('@react-native-async-storage/async-storage/jest/async-storage-mock')
);

// Silence the warning: Animated: `useNativeDriver` is not supported
jest.mock('react-native/Libraries/Animated/NativeAnimatedHelper');